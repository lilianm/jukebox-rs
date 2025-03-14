//! # Jukebox Decoder
//!
//! This crate provides functionality for decoding audio streams into frames.
//! It defines traits for streams of frames and decoders that can process byte buffers
//! into these streams. The main components are:
//!
//! - `Stream`: A trait representing a stream of frames.
//! - `Decoder`: A trait representing a decoder that can decode a buffer of bytes into a stream of frames.
//!
//! ## Example
//!
//! ```rust
//! use bytes::Bytes;
//! use jukebox_decoder::{Decoder, Frame, Stream};
//!
//! struct MyDecoder;
//!
//! impl Decoder for MyDecoder {
//!     fn name() -> &'static str {
//!         "MyDecoder"
//!     }
//!
//!     fn decode(buf: Bytes) -> Box<dyn Stream> {
//!         // Implementation goes here
//!         Box::new(MyStream { data: buf.clone() })
//!     }
//! }
//!
//! #[derive(Default)]
//! struct MyStream {
//!     // Stream implementation details
//!     data: Bytes,
//! }
//!
//! impl Iterator for MyStream {
//!     type Item = Frame;
//!
//!     fn next(&mut self) -> Option<Self::Item> {
//!         // Implementation goes here
//!         None
//!     }
//! }
//!
//! impl Stream for MyStream {}
//! ```

use bytes::Bytes;

mod error;
mod frame;

pub use error::Error;
pub use frame::Frame;

/// A trait representing a stream of frames.
pub trait Stream: Iterator<Item = Frame> + Sync + Send {}

/// A trait representing a decoder that can decode a buffer of bytes into a stream of frames.
pub trait Decoder {
    fn name() -> &'static str;
    fn decode(buf: Bytes) -> Box<dyn Stream>;
}
