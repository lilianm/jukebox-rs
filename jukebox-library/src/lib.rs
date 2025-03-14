#![allow(async_fn_in_trait)]

pub use jukebox_decoder::Stream;

pub type LibraryId = usize;

pub trait Library: Send + Clone {
    async fn random(&self) -> (LibraryId, Box<dyn Stream>);
    async fn get(&self, id: LibraryId) -> Option<Box<dyn Stream>>;
    // TODO add search
    // TODO add id selection
    // TODO split library and input (http, file, s3, ...)
}
