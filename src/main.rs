mod api;
mod subhandlers;
mod utility;
mod models;
use std::{error::Error, path::PathBuf};
use std::io::Write;
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

    env_logger::Builder::new()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "{2} {1:<5} {0}",
                record.args(),
                record.level(),
                ts,
            )
        })
        .filter(None, match opt.verbose {
            0 => log::LevelFilter::Error,
            1 => log::LevelFilter::Warn,
            2 => log::LevelFilter::Info,
            3 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let r = match &opt.cmd {
        Command::Album { .. } => subhandlers::album_handler::AlbumHandler::handle(&opt).await,
    };

    match r {
        Ok(_) => (),
        Err(e) => log::error!("{}", e),
    }

    Ok(())
}
