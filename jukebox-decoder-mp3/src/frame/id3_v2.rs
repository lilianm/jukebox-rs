use bytes::{Buf, Bytes};

pub(crate) struct Id3V2 {
    // data: Bytes,
}

const HEADER_SIZE: usize = 10;

impl TryFrom<&mut Bytes> for Id3V2 {
    type Error = jukebox_decoder::Error;
    fn try_from(value: &mut Bytes) -> Result<Self, Self::Error> {
        match value.chunk() {
            &[b'I', b'D', b'3', _v1, _v2, _flag, s1, s2, s3, s4, ..] => {
                let size =
                    ((s1 as u32) << 21) | ((s2 as u32) << 14) | ((s3 as u32) << 7) | (s4 as u32);
                let size = size as usize;
                if size > value.len() {
                    Err(Self::Error::InvalidData)
                } else {
                    value.advance(size + HEADER_SIZE);
                    Ok(Id3V2 {})
                }
            }
            _ => Err(Self::Error::InvalidData),
        }
    }
}
