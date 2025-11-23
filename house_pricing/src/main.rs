use clap::Parser;
use file_utility::{self, arg_parsing::Cli};
fn main() {
    // if let Some(err) = Cli::try_parse().err() {
    //     dbg!(err.source());
    // }
    Cli::parse();
}
