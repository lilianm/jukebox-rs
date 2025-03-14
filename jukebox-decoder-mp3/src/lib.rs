//! # MP3 Decoder
//!
//! This crate provides functionality for decoding MP3 audio streams into frames.
//! It defines modules for the decoder, frame, and stream handling. The main components are:
//!
//! - `decoder` Contains the `Mp3Decoder` struct which implements the [`Decoder`](jukebox_decoder::Decoder) trait for mp3 format.
//!
//! ## Example
//!
//! ```rust
//! use bytes::Bytes;
//! use jukebox_decoder_mp3::Decoder;
//!
//! let decoder = Decoder::new();
//! let bytes = Bytes::from(vec![/* MP3 data */]);
//! let stream = decoder.decode(bytes);
//! ```

mod decoder;
mod frame;
mod stream;

pub use decoder::Mp3Decoder as Decoder;
