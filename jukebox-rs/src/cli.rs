use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Jukebox")]
#[command(author = "Author Name <lilian-code@maurel.biz>")]
#[command(version = "1.0")]
#[command(about = "Jukebox application")]
pub(crate) struct Cli {
    /// Listen port
    #[arg(short, long, default_value = "8080", env = "PORT")]
    pub port: u16,
}
