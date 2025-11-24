mod custom_pipeline;

use std::path::Path;

use clap::Parser;
use file_utility::{self, arg_parsing::Cli};
use polars::prelude::*;
fn main() {
    let data_path = Cli::parse().data_path;

    let data_path: Arc<Path> = Arc::from(data_path);

    let reader = LazyCsvReader::new(PlPath::Local(data_path));
}
