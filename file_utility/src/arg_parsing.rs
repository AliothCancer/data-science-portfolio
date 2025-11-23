use clap::Parser;
use core::fmt;
use std::{error::Error, fmt::Debug, fs::File, path::PathBuf};

#[derive(Parser)]
#[command(
    author = "AliothCancer",
    version,
    about = "Run the program for a specific CSV data file"
)]
#[derive(Debug)]
pub struct Cli {
    /// The CSV file containing the data
    #[arg(short = 'p', long = "data_path", value_name = "data path", value_parser=custom_csv_validator)]
    data_path: PathBuf,
}

#[derive(Debug)]
pub enum LocalCsvError {
    PathUnreachable(String), // Permission denied, etc.
    PathNotExists,
    NotAFile,             // Flattens Exist::NotAFile
    NotACsv,              // Flattens IsFile::NotACsv
    BadStructure(String), // Flattens IsCsv::BadStructure
}
impl Error for LocalCsvError {}
// We implement Display to define how these errors look to the user
impl fmt::Display for LocalCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathNotExists => write!(f, "The path provided does not exist."),
            Self::PathUnreachable(e) => write!(f, "System access error: {}", e),
            Self::NotAFile => write!(
                f,
                "The path exists but it is not a file (is it a directory?)."
            ),
            Self::NotACsv => write!(f, "The file extension suggests this is not a CSV."),
            Self::BadStructure(details) => {
                write!(f, "CSV parsing failed. Metadata check error: {}", details)
            }
        }
    }
}

fn custom_csv_validator(path_str: &str) -> Result<PathBuf, LocalCsvError> {
    let path = PathBuf::from(path_str);

    // 1. Check Existence / Reachability
    match path.try_exists() {
        Ok(true) => {} // Continue
        Ok(false) => return Err(LocalCsvError::PathNotExists),
        Err(e) => return Err(LocalCsvError::PathUnreachable(e.to_string())),
    }

    // 2. Check if File
    if !path.is_file() {
        return Err(LocalCsvError::NotAFile);
    }

    // 3. Check Extension (Optional, but good for IsFile -> IsCsv logic)
    // This covers your "NotACsv" variant regarding file identity
    if let Some(ext) = path.extension() {
        if ext != "csv" {
            return Err(LocalCsvError::NotACsv);
        }
    } else {
        return Err(LocalCsvError::NotACsv);
    }

    // 4. Check Structure (Header Metadata)
    // This covers your "BadStructure" variant
    let file = File::open(&path).map_err(|e| LocalCsvError::PathUnreachable(e.to_string()))?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    match rdr.headers() {
        Ok(_) => Ok(path), // Success!
        Err(e) => Err(LocalCsvError::BadStructure(e.to_string())),
    }
}
