use jukebox_library::{Library, LibraryId};
use jukebox_playlist::{Playlist, Stream};
use std::default;

#[derive(Debug, Clone)]
pub struct PlaylistRandom<T: Library> {
    current: Option<LibraryId>,
    library: T,
}

impl<T> PlaylistRandom<T>
where
    T: Library,
{
    pub fn new(library: T) -> Self {
        Self {
            library,
            current: None,
        }
    }
}

impl<T> Playlist for PlaylistRandom<T>
where
    T: Library,
{
    async fn next(&mut self) -> Box<dyn Stream> {
        let (song_id, stream) = self.library.random().await;
        self.current = Some(song_id);
        stream
    }

    async fn prev(&mut self) -> Box<dyn Stream> {
        self.next().await
    }

    async fn rewind(&mut self) -> Box<dyn Stream> {
        if let Some(id) = self.current {
            if let Some(stream) = self.library.get(id).await {
                return stream;
            }
        }

        self.next().await
    }
}
