use std::{ops::AddAssign, path::Path};

use jukebox_decoder::Decoder;

use crate::{Library, file::LibraryFileInner};

#[derive(Debug)]
pub struct LibraryFileBuilder<D>
where
    D: Decoder,
{
    pub(crate) inner: LibraryFileInner,
    _marker: std::marker::PhantomData<D>,
}

impl<D> Default for LibraryFileBuilder<D>
where
    D: Decoder,
{
    fn default() -> Self {
        Self {
            inner: LibraryFileInner::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D> LibraryFileBuilder<D>
where
    D: Decoder,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Library<D> {
        self.into()
    }
}

impl<D, P> AddAssign<P> for LibraryFileBuilder<D>
where
    P: AsRef<Path>,
    D: Decoder,
{
    fn add_assign(&mut self, directory: P) {
        let _ = self.inner.add(directory);
    }
}
