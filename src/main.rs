mod url_builder;
mod api;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "imgur-dl")]
pub struct Opt {
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u64,

    #[structopt(short, long)]
    follow_order: bool,

    album_id: String,
}

fn main() {
    println!("Hello, world!");
}
