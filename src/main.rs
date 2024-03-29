mod api;
mod subhandlers;
mod models;
mod mime2ext;
use std::{error::Error, path::PathBuf};
use std::io::Write;
use structopt::StructOpt;
use subhandlers::handler_traits::Handler;

#[derive(StructOpt)]
#[structopt(name = "imgur-dl")]
pub struct Opt {
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u64,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
pub struct AlbumDownloadOptions {
    #[structopt(required = true, min_values = 1)]
    album_hashes: Vec<String>,
    #[structopt(short = "-o", long, parse(from_os_str))]
    output_directory: Option<PathBuf>,
    #[structopt(short = "-t", long, default_value = "[%(id)] %(title)")]
    output_template: String,
}

#[derive(StructOpt)]
pub struct AlbumInfoOptions {
    album_hash: String,
    #[structopt(short = "-t", long, default_value = "[%(id)] %(title)")]
    output_template: String,
}

#[derive(StructOpt)]
pub enum AlbumCommand {
    #[structopt(help = "Download an album")]
    Download(AlbumDownloadOptions),
    #[structopt(help = "List information for an album")]
    Info(AlbumInfoOptions),
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "album")]
    Album(AlbumCommand),
    Upload {
        #[structopt(short = "-a", long)]
        api_key: String,
    },
    Config {
    },
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
        Command::Upload { .. } => subhandlers::upload_handler::UploadHandler::handle(&opt).await,
        Command::Config { .. } => subhandlers::config_handler::ConfigHandler::handle(&opt).await,
    };

    match r {
        Ok(_) => (),
        Err(e) => log::error!("{}", e),
    }

    Ok(())
}
