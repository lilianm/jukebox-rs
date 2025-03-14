use actix_web::{App, HttpServer, web};
use clap::Parser as _;
use std::borrow::Borrow;
use tracing::info;

use jukebox_channel::{ChannelCommand, ChannelManager};
use jukebox_decoder_mp3::Decoder as Mp3Decoder;
use jukebox_playlist_random::Playlist as PlaylistRandom;

mod cli;
mod command;
mod stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = cli::Cli::parse();

    let mut library = jukebox_library_file::Builder::<Mp3Decoder>::new();
    for path in &args.file_urls {
        library += path;
    }
    let library = library.build();
    let playlist = PlaylistRandom::new(library);
    let mut channel_manager = ChannelManager::new(playlist);

    let channel_subscriber: ChannelCommand = channel_manager.borrow().into();
    tokio::spawn(async move { channel_manager.run().await });

    info!("Starting Jukebox on port {}", args.port);

    HttpServer::new(move || {
        let data_channel_manager = web::Data::new(channel_subscriber.clone());
        App::new()
            .app_data(data_channel_manager)
            .route("/api/stream", web::get().to(stream::api_stream))
            .route("/api/next", web::get().to(command::api_next))
            .route("/api/previous", web::get().to(command::api_previous))
    })
    .bind(("::", args.port))?
    .run()
    .await?;

    Ok(())
}
