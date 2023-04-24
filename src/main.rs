mod utility;
mod api;
mod subhandlers;
mod album;

use std::error::Error;

use structopt::StructOpt;
use subhandlers::handler_traits::Handler;

#[derive(Debug, StructOpt)]
#[structopt(name = "imgur-dl")]
pub struct Opt {
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u64,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Album")]
    Album {
        album_hash: String,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let r = match &opt.cmd {
        Command::Album { .. } => subhandlers::album_handler::AlbumHandler::handle(&opt).await,
    };

    Ok(())
}
