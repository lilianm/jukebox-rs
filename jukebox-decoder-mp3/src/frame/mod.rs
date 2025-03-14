use std::cell::RefCell;

use bytes::{Buf, Bytes};

use jukebox_decoder::Frame as DecoderFrame;

mod id3_v1;
mod id3_v2;
mod mp3;

pub(crate) enum Frame {
    Id3V1(id3_v1::Id3V1),
    Id3V2(id3_v2::Id3V2),
    Mp3(mp3::Mp3Frame),
}

impl Frame {
    pub(super) fn decoder(
        data: &mut Bytes,
    ) -> Result<Option<DecoderFrame>, jukebox_decoder::Error> {
        let mut data = RefCell::new(data);
        loop {
            match Self::decode_one_frame(data.get_mut())? {
                Frame::Id3V1(_) => continue, // Ignore metadata
                Frame::Id3V2(_) => continue, // Ignore metadata
                Frame::Mp3(frame) => return Ok(Some(frame.into())),
            }
        }
    }

    fn decode_one_frame(data: &mut Bytes) -> Result<Frame, jukebox_decoder::Error> {
        match data.chunk() {
            [b'T', b'A', b'G', ..] => id3_v1::Id3V1::try_from(data).map(Frame::Id3V1),
            [b'I', b'D', b'3', ..] => id3_v2::Id3V2::try_from(data).map(Frame::Id3V2),
            [0xFF, ..] => mp3::Mp3Frame::try_from(data).map(Frame::Mp3),
            _ => Err(jukebox_decoder::Error::InvalidData),
        }
    }
}
