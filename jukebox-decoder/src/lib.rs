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
//!     fn name(&self) -> &'static str {
//!         "MyDecoder"
//!     }
//!
//!     fn decode(&self, buf: Bytes) -> Box<dyn Stream> {
//!         // Implementation goes here
//!         Box::new(MyStream::new(buf))
//!     }
//! }
//!
//! struct MyStream {
//!     // Stream implementation details
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
///
/// # Required Methods
///
/// - `name`: Returns the name of the decoder as a static string slice.
/// - `decode`: Takes a buffer of bytes and returns a boxed stream of frames.
///
/// # Example
///
/// ```
/// use bytes::Bytes;
/// use jukebox_decoder::{Decoder, Frame, Stream};
///
/// struct MyDecoder;
///
/// impl Decoder for MyDecoder {
///     fn name(&self) -> &'static str {
///         "MyDecoder"
///     }
///
///     fn decode(&self, buf: Bytes) -> Box<dyn Stream> {
///         // Implementation goes here
///         Box::new(MyStream::new(buf))
///     }
/// }
///
/// struct MyStream {
///     // Stream implementation details
/// }
///
/// impl Iterator for MyStream {
///     type Item = Frame;
///
///     fn next(&mut self) -> Option<Self::Item> {
///         // Implementation goes here
///         None
///     }
/// }
///
/// impl Stream for MyStream {}
/// ```
pub trait Decoder {
    fn name() -> &'static str;
    fn decode(buf: Bytes) -> Box<dyn Stream>;
}
