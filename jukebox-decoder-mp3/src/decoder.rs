use bytes::Bytes;

use jukebox_decoder::{Decoder, Stream};

use super::stream::Mp3Stream;

#[derive(Default)]
pub struct Mp3Decoder {}

impl Decoder for Mp3Decoder {
    fn name() -> &'static str {
        "mp3"
    }

    fn decode(buf: Bytes) -> Box<dyn Stream> {
        Box::new(Mp3Stream::new(buf))
    }
}
