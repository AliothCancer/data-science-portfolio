#![allow(unused)]

use std::path::PathBuf;

use polars::prelude::LazyCsvReader;

pub struct LocalFile {
    path: PathBuf,
    config: LazyCsvReader,
}

impl LocalFile {
    pub fn new(path: PathBuf, config: LazyCsvReader) -> Self {
        Self { path, config }
    }
}
