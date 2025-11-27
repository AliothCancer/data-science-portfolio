mod custom_pipeline;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use file_utility::{self, arg_parsing::Cli};
use polars::prelude::*;
fn main() -> Result<(), Box<dyn Error>> {
    let Cli {
        data_path,
        feature_desc_path,
    } = Cli::parse();

    let df_data = read_and_return_df(data_path)?;
    let df_features = read_and_return_df(feature_desc_path)?;

    let feat2 = df_features.select([col("name")]);
    dbg!(feat2.collect()?);
    Ok(())
}

fn read_and_return_df(path: PathBuf) -> Result<LazyFrame, PolarsError>{
    LazyCsvReader::new(PlPath::Local(Arc::from(path)))
        .with_has_header(true)
        .with_ignore_errors(false)
        .with_infer_schema_length(Some(200))
        .with_missing_is_null(true)
        .with_null_values(Some(NullValues::AllColumns(vec!["NA".into()])))
        .finish()
}