use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    convert::Infallible,
    ops::Deref,
    sync::{Arc, Weak},
    task::{Poll, Waker},
};

use actix_web::body::{BodySize, MessageBody};
use bytes::Bytes;
use pin_project::pin_project;
use tokio::sync::RwLock;

#[derive(Default)]
struct StreamRaw {
    waker: Option<Waker>,
    frames: VecDeque<Bytes>,
}

#[derive(Default)]
struct StreamLock {
    inner: RwLock<StreamRaw>,
}

impl Deref for StreamLock {
    type Target = RwLock<StreamRaw>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone)]
pub struct StreamWeak {
    inner: Weak<StreamLock>,
}

#[derive(Default)]
#[pin_project]
pub struct Stream {
    inner: Arc<StreamLock>,
}

impl From<&Stream> for StreamWeak {
    fn from(value: &Stream) -> Self {
        Self {
            inner: Arc::downgrade(&value.inner),
        }
    }
}

impl StreamRaw {
    fn push(&mut self, data: &Bytes) {
        self.frames.push_back(data.clone());
        self.waker.as_ref().map(Waker::wake_by_ref);
    }

    fn get(&mut self, cx: &mut std::task::Context) -> Option<Bytes> {
        match self.frames.borrow_mut().pop_front() {
            None => {
                // No data available
                self.waker = Some(cx.waker().clone());
                None
            }
            v => v,
        }
    }
}

impl StreamWeak {
    pub(crate) async fn push(&self, data: &Bytes) {
        if let Some(stream) = self.inner.upgrade() {
            stream.inner.write().await.push(data);
        }
    }

    pub(crate) fn active(&self) -> bool {
        self.inner.strong_count() != 0
    }
}

impl MessageBody for Stream {
    type Error = Infallible;

    fn size(&self) -> BodySize {
        BodySize::Stream
    }

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let stream = self.project().inner;
        let mut res = Box::pin(stream.inner.write());
        match res.as_mut().poll(cx) {
            Poll::Ready(mut guard) => guard
                .get(cx)
                .map(|d| Poll::Ready(Some(Ok(d))))
                .unwrap_or(Poll::Pending),
            Poll::Pending => Poll::Pending,
        }
    }
}
