use std::{collections::HashMap, time::Duration};

use jukebox_playlist::Playlist;
use tokio::{sync::mpsc, time::Instant};

use crate::{
    channel::{Channel, ChannelAction},
    stream::Stream,
};

pub struct ChannelManager<T: Playlist> {
    incoming: mpsc::Receiver<ChannelMessage>,
    subcriber: mpsc::Sender<ChannelMessage>,

    channels: HashMap<String, Channel<T>>,
    playlist: T,
}

struct ChannelMessage {
    name: String,
    action: ChannelAction,
}

#[derive(Clone)]
pub struct ChannelCommand {
    channel: mpsc::Sender<ChannelMessage>,
}

impl ChannelCommand {
    pub async fn register(&self, name: impl AsRef<str>, st: &Stream) -> Result<(), std::io::Error> {
        self.channel
            .send(ChannelMessage {
                name: name.as_ref().to_string(),
                action: ChannelAction::Register(st.into()),
            })
            .await
            .map_err(|_| std::io::ErrorKind::BrokenPipe.into())
    }

    pub async fn next(&self, name: impl AsRef<str>) -> Result<(), std::io::Error> {
        self.channel
            .send(ChannelMessage {
                name: name.as_ref().to_string(),
                action: ChannelAction::Next,
            })
            .await
            .map_err(|_| std::io::ErrorKind::BrokenPipe.into())
    }

    pub async fn previous(&self, name: impl AsRef<str>) -> Result<(), std::io::Error> {
        self.channel
            .send(ChannelMessage {
                name: name.as_ref().to_string(),
                action: ChannelAction::Previous,
            })
            .await
            .map_err(|_| std::io::ErrorKind::BrokenPipe.into())
    }

    pub async fn rewind(&self, name: impl AsRef<str>) -> Result<(), std::io::Error> {
        self.channel
            .send(ChannelMessage {
                name: name.as_ref().to_string(),
                action: ChannelAction::Rewind,
            })
            .await
            .map_err(|_| std::io::ErrorKind::BrokenPipe.into())
    }
}

impl<T> From<&ChannelManager<T>> for ChannelCommand
where
    T: Playlist,
{
    fn from(value: &ChannelManager<T>) -> Self {
        Self {
            channel: value.subcriber.clone(),
        }
    }
}

impl<T> ChannelManager<T>
where
    T: Playlist,
{
    const CHANNEL_REFRESH: u32 = 100_000_000; // 100 ms
    pub fn new(playlist: T) -> Self {
        let (subcriber, incoming) = mpsc::channel(128);
        Self {
            incoming,
            subcriber,
            channels: Default::default(),
            playlist,
        }
    }

    pub async fn run(&mut self) {
        let duration = Duration::new(0, Self::CHANNEL_REFRESH);
        let mut next = Instant::now();

        loop {
            tokio::select! {
                _ = tokio::time::sleep_until(next) => {
                    for (_, channel) in self.channels.iter_mut() {
                        channel.run(next).await;
                    }
                    next += duration;
                }
                Some(msg) = self.incoming.recv() => {
                    self.channels
                        .entry(msg.name)
                        .or_insert_with(|| Channel::new(self.playlist.clone()))
                        .action(msg.action).await;
                }
            }
        }
    }
}
