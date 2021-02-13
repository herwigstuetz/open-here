mod cli;

use structopt::StructOpt;

fn main() {
    let args = cli::Args::from_args();

    println!("{:?}", args);
}
