use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    ops::{AddAssign, Sub},
    time::Duration,
};

use jukebox_decoder::{Frame, Stream};
use jukebox_playlist::Playlist;
use tokio::time::Instant;
use tracing::{info, trace};

use crate::StreamWeak;

#[derive(Clone)]
struct ChannelTime {
    start: Instant,
    frames: HashMap<usize, usize>,
}

#[derive(Default)]
pub struct Channel<T>
where
    T: Playlist,
{
    playlist: T,

    data: Option<Box<dyn Stream>>,
    time: ChannelTime,
    start_time: ChannelTime,
    pause_time: Option<Instant>,

    streams: Vec<StreamWeak>,
}

pub enum ChannelAction {
    Register(StreamWeak),
    Next,
    Previous,
    Rewind,
}

impl Debug for ChannelAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ChannelAction::Register(_) => write!(f, "Register"),
            ChannelAction::Next => write!(f, "Next"),
            ChannelAction::Previous => write!(f, "Previous"),
            ChannelAction::Rewind => write!(f, "Rewind"),
        }
    }
}

impl Default for ChannelTime {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            frames: Default::default(),
        }
    }
}

impl ChannelTime {
    fn now(&self) -> Instant {
        let mut value = self.start;

        self.frames
            .iter()
            .map(|(sample_rate, nb_sample)| {
                Duration::from_micros((nb_sample * 1_000_000 / sample_rate) as u64)
            })
            .for_each(|d| value += d);
        value
    }

    fn resync(&mut self, episilon: Duration) {
        self.start += episilon;
    }
}

impl AddAssign<&Frame> for ChannelTime {
    fn add_assign(&mut self, rhs: &Frame) {
        *self.frames.entry(rhs.sample_rate).or_default() += rhs.nb_samples;
    }
}

impl Sub<&ChannelTime> for &ChannelTime {
    type Output = Duration;

    fn sub(self, rhs: &ChannelTime) -> Self::Output {
        self.now() - rhs.now()
    }
}

impl<T> Channel<T>
where
    T: Playlist,
{
    pub(crate) fn new(playlist: T) -> Self {
        let now = ChannelTime::default();
        Self {
            playlist,

            pause_time: Some(now.start),
            time: now.clone(),
            start_time: now,
            data: Default::default(),
            streams: Default::default(),
        }
    }

    pub(crate) fn register(&mut self, stream: StreamWeak) {
        self.streams.push(stream)
    }

    pub(crate) async fn run(&mut self, now: Instant) {
        self.streams = self
            .streams
            .iter()
            .filter(|e| e.active())
            .cloned()
            .collect();

        match (&self.pause_time, self.streams.is_empty()) {
            (None, true) => {
                // Stop stream
                self.pause_time = Some(now);
                return;
            }
            (Some(_), true) => {
                return;
            }
            (None, false) => {}
            (Some(pause_time), false) => {
                // Restart stream
                let duration = now - *pause_time;
                self.start_time.resync(duration);
                self.time.resync(duration);
                self.pause_time = None
            }
        }

        trace!("channel: nb stream {}", self.streams.len());

        while self.time.now() < now {
            if self.data.is_none() {
                self.action(ChannelAction::Next).await;
            }
            let decoder = self.data.as_mut().unwrap();
            match decoder.next() {
                Some(frame) => {
                    self.time += &frame;
                    for stream in self.streams.iter() {
                        stream.push(frame.as_ref()).await;
                    }
                }
                None => {
                    self.data = None;
                }
            }
        }
        info!("channel: position {:?}", &self.time - &self.start_time);
    }

    pub(crate) async fn action(&mut self, action: ChannelAction) {
        println!("channel: action {:?}", action);
        info!("channel: action {:?}", action);
        match action {
            ChannelAction::Register(stream) => self.register(stream),
            ChannelAction::Next => {
                let data = self.playlist.next().await;
                self.update_decoder(data)
            }
            ChannelAction::Previous => {
                let data = self.playlist.prev().await;
                self.update_decoder(data)
            }
            ChannelAction::Rewind => {
                let data = self.playlist.rewind().await;
                self.update_decoder(data)
            }
        };
    }

    fn update_decoder(&mut self, data: Box<dyn Stream>) {
        self.data = Some(data);
        self.start_time = self.time.clone();
    }
}
