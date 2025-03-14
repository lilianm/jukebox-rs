use bytes::{Buf, Bytes};

pub(crate) struct Id3V1 {
    // data: Bytes,
}

const ID3V1_SIZE: usize = 128;

impl TryFrom<&mut Bytes> for Id3V1 {
    type Error = jukebox_decoder::Error;
    fn try_from(value: &mut Bytes) -> Result<Self, Self::Error> {
        match value.chunk() {
            &[b'T', b'A', b'G', ..] => {
                if value.len() > ID3V1_SIZE {
                    Err(Self::Error::InvalidData)
                } else {
                    value.advance(ID3V1_SIZE);
                    Ok(Self {})
                }
            }
            _ => Err(Self::Error::InvalidData),
        }
    }
}
