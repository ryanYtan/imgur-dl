mod utility;
mod api;
mod subhandlers;
mod models;

use std::{error::Error, path::PathBuf};

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

        #[structopt(short = "-o", long, parse(from_os_str))]
        output_directory: Option<PathBuf>,
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
