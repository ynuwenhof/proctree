mod process;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    sort: bool,
}

fn main() {
    let cli = Cli::parse();
}
