use bytes::Bytes;

use jukebox_decoder::Stream;

#[derive(Default)]
pub struct Mp3Stream {
    data: Bytes,
}

impl Stream for Mp3Stream {}

impl Mp3Stream {
    pub(super) fn new(buf: Bytes) -> Self {
        Self { data: buf }
    }
}

impl Iterator for Mp3Stream {
    type Item = jukebox_decoder::Frame;

    fn next(&mut self) -> Option<Self::Item> {
        super::frame::Frame::decoder(&mut self.data).unwrap_or_default()
    }
}
