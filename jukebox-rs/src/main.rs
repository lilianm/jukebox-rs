use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clap::Parser as _;
use tracing::info;

mod cli;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = cli::Cli::parse();

    info!("Starting Jukebox on port {}", args.port);

    HttpServer::new(|| App::new().route("/api/stream", web::get().to(index)))
        .bind(("::", args.port))?
        .run()
        .await?;

    Ok(())
}
