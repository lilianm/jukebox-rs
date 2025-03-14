use bytes::Bytes;

#[derive(Debug)]
pub struct Frame {
    pub data: Bytes,
    pub nb_samples: usize,
    pub sample_rate: usize,
}

impl AsRef<Bytes> for Frame {
    fn as_ref(&self) -> &Bytes {
        &self.data
    }
}

impl Frame {
    pub fn new(data: Bytes, nb_samples: usize, sample_rate: usize) -> Self {
        Self {
            data: data.clone(),
            nb_samples,
            sample_rate,
        }
    }
}
