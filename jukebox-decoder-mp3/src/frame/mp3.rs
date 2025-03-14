use bytes::{Buf, Bytes};
use jukebox_decoder::Frame;

pub(crate) struct Mp3Frame {
    data: Frame,
}

impl From<Mp3Frame> for Frame {
    fn from(value: Mp3Frame) -> Self {
        value.data
    }
}

impl TryFrom<&mut Bytes> for Mp3Frame {
    type Error = jukebox_decoder::Error;

    fn try_from(value: &mut Bytes) -> Result<Self, Self::Error> {
        match value.chunk() {
            &[0xFF, h1, h2, _, ..] if h1 & 0xE0 == 0xE0 => {
                let (bitrate_band_idx, sampling_shift, nb_frames, padding_bytes) =
                    match ((h1 >> 3) & 0x03, ((h1 >> 1) & 0x03)) {
                        // Mpeg 1
                        (3, 3) => (1, 2, 384, 4),  // Layer I
                        (3, 2) => (2, 2, 1152, 1), // Layer II
                        (3, 1) => (3, 2, 1152, 1), // Layer III
                        // Mpeg 2
                        (2, 3) => (4, 1, 384, 4),  // Layer I
                        (2, 2) => (5, 1, 1152, 1), // Layer II
                        (2, 1) => (5, 1, 576, 1),  // Layer III
                        // Mpeg 2.5
                        (0, 3) => (4, 0, 384, 4),  // Layer I
                        (0, 2) => (5, 0, 1152, 1), // Layer II
                        (0, 1) => (5, 0, 576, 1),  // Layer III
                        // Invalid
                        _ => (0, 0, 0, 0),
                    };

                const BITRATE_BAND: [[u16; 16]; 6] = [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [
                        0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0,
                    ],
                    [
                        0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0,
                    ],
                    [
                        0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0,
                    ],
                    [
                        0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0,
                    ],
                    [
                        0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0,
                    ],
                ];
                let bitrate = BITRATE_BAND[bitrate_band_idx as usize][(h2 >> 4) as usize] as u32;

                let (sampling_rate, sampling_rate_compute) = match (h2 >> 2) & 0x3 {
                    0 => (
                        (11025 << sampling_shift) as u32,
                        (4000 << 16) / (8 * 11025) as u32,
                    ),
                    1 => ((12000 << sampling_shift), (4000 << 16) / (8 * 12000)),
                    2 => ((8000 << sampling_shift), (4000 << 16) / (8 * 8000)),
                    _ => (0, 0),
                };

                let mut size =
                    (sampling_rate_compute * (bitrate >> 2) * nb_frames) >> (16 + sampling_shift);
                if (h2 & 0x02) == 0x02 {
                    // Check padding bit
                    size += padding_bytes;
                }
                Ok(Mp3Frame {
                    data: Frame::new(
                        value.split_to(size as usize),
                        nb_frames as usize,
                        sampling_rate as usize,
                    ),
                })
            }
            _ => Err(Self::Error::InvalidData),
        }
    }
}
