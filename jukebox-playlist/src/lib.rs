#![allow(async_fn_in_trait)]

pub use jukebox_decoder::Stream;

pub trait Playlist: Clone + Send {
    async fn next(&mut self) -> Box<dyn Stream>;
    async fn prev(&mut self) -> Box<dyn Stream>;
    async fn rewind(&mut self) -> Box<dyn Stream>;
}
