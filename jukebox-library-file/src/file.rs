use std::{io, ops::Deref, path::Path, sync::Arc};

use bytes::Bytes;

use jukebox_decoder::Decoder;
use jukebox_library::{Library, LibraryId, Stream};
use rand::Rng;

use crate::Builder;

#[derive(Debug, Default)]
pub struct LibraryFileInner {
    files: Vec<String>,
}

#[derive(Debug)]
pub struct LibraryFile<D>
where
    D: Decoder,
{
    inner: Arc<LibraryFileInner>,
    _marker: std::marker::PhantomData<D>,
}

impl<D> Deref for LibraryFile<D>
where
    D: Decoder,
{
    type Target = LibraryFileInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LibraryFileInner {
    pub(crate) fn add(&mut self, directory: impl AsRef<Path>) -> Result<(), io::Error> {
        std::fs::read_dir(directory)?
            .filter_map(Result::ok)
            .filter(|file| file.metadata().map(|m| m.is_file()).unwrap_or(false))
            .for_each(|file| {
                self.files.push(file.path().to_string_lossy().to_string());
            });
        Ok(())
    }
}

impl<D> Library for LibraryFile<D>
where
    D: Decoder,
{
    async fn random(&self) -> (LibraryId, Box<dyn Stream>) {
        let index = rand::rng().random_range(0..self.files.len());
        let filename = &self.files[index];
        (
            index,
            D::decode(Bytes::from(tokio::fs::read(filename).await.unwrap())),
        )
    }

    async fn get(&self, id: LibraryId) -> Option<Box<dyn Stream>> {
        let file = self.files.get(id)?;
        Some(D::decode(Bytes::from(tokio::fs::read(file).await.unwrap())))
    }
}

impl<D> Clone for LibraryFile<D>
where
    D: Decoder,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

// LibraryFile is safe but Decoder don't need to be Sync or Send
unsafe impl<D> Sync for LibraryFile<D> where D: Decoder {}
unsafe impl<D> Send for LibraryFile<D> where D: Decoder {}

impl<D> From<Builder<D>> for LibraryFile<D>
where
    D: Decoder,
{
    fn from(builder: Builder<D>) -> Self {
        Self {
            inner: Arc::new(builder.inner),
            _marker: std::marker::PhantomData,
        }
    }
}
