use clap::Parser;
use core::fmt;
use std::{error::Error, fmt::Debug, path::PathBuf};

#[derive(Parser)]
#[command(
    author = "AliothCancer",
    version,
    about = "Run the program for a specific CSV data file"
)]
#[derive(Debug)]
pub struct Cli {
    /// The CSV file containing the data
    #[arg(short = 'p', long = "data_path", value_name = "data path", value_parser=custom_csv_path_validator)]
    pub data_path: PathBuf,
    #[arg(short = 'f', long = "feature_path", value_name = "feature description", value_parser=custom_csv_path_validator)]
    pub feature_desc_path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum LocalCsvError {
    PathUnreachable(String), // Permission denied, etc.
    PathNotExists,
    NotAFile, // Flattens Exist::NotAFile
    NotACsv,  // Flattens IsFile::NotACsv
    MissingExtension,
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
            Self::MissingExtension => write!(f, "Extension file is missing, should be .csv"),
        }
    }
}

fn custom_csv_path_validator(path_str: &str) -> Result<PathBuf, LocalCsvError> {
    let path = PathBuf::from(path_str);

    // 1. Check Existence / Reachability
    match path.try_exists() {
        Ok(true) => {} // pass to next checks
        Ok(false) => return Err(LocalCsvError::PathNotExists),
        Err(e) => return Err(LocalCsvError::PathUnreachable(e.to_string())),
    }

    // 2. Check if File
    if !path.is_file() {
        return Err(LocalCsvError::NotAFile);
    }

    // 3. Check Extension
    if let Some(ext) = path.extension() {
        if ext != "csv" {
            Err(LocalCsvError::NotACsv)
        } else {
            Ok(path)
        }
    } else {
        Err(LocalCsvError::MissingExtension)
    }
}

// Implementation for fun
// struct UnverifiedPath(PathBuf);
// struct IsReachableAndExists(PathBuf);
// struct IsFile(PathBuf);
// struct HasCsvExtension(PathBuf);

// fn custom_validation_traits(path: &str) -> Result<PathBuf, LocalCsvError> {
//     UnverifiedPath(path.into())
//         .try_validate::<IsReachableAndExists>()?
//         .try_validate::<IsFile>()?
//         .try_validate::<HasCsvExtension>()
//         .map(|x| x.0)
// }

// trait ToBeValidated {
//     fn id() -> usize;
// }
// impl ToBeValidated for UnverifiedPath {
//     fn id() -> usize {
//         0
//     }
// }
// impl ToBeValidated for IsReachableAndExists {
//     fn id() -> usize {
//         1
//     }
// }
// impl ToBeValidated for IsFile {
//     fn id() -> usize {
//         2
//     }
// }
// impl ToBeValidated for HasCsvExtension {
//     fn id() -> usize {
//         3
//     }
// }
// trait Validate: ToBeValidated
// where
//     Self: Sized,
// {
//     fn map_results<NextCheck: ToBeValidated + From<Self>>(self)
//     -> Result<NextCheck, LocalCsvError>;
//     fn try_validate<NextCheck: ToBeValidated + From<Self>>(
//         self,
//     ) -> Result<NextCheck, LocalCsvError> {
//         assert_eq!(NextCheck::id(), Self::id() + 1);
//         self.map_results()
//     }
// }

// impl Validate for UnverifiedPath {
//     fn map_results<NextCheck: ToBeValidated + From<Self>>(
//         self,
//     ) -> Result<NextCheck, LocalCsvError> {
//         match self.0.try_exists() {
//             Ok(true) => Ok(NextCheck::from(self)),
//             Ok(false) => Err(LocalCsvError::PathNotExists),
//             Err(e) => Err(LocalCsvError::PathUnreachable(e.to_string()))
//         }
//     }
// }

// impl Validate for IsReachableAndExists {
//     fn map_results<NextCheck: ToBeValidated + From<Self>>(
//         self,
//     ) -> Result<NextCheck, LocalCsvError> {
//         match self.0.try_exists() {
//             Ok(true) => Ok(NextCheck::from(self)),
//             Ok(false) => Err(LocalCsvError::PathNotExists),
//             Err(e) => Err(LocalCsvError::PathUnreachable(e.to_string())),
//         }
//     }
// }

// impl Validate for IsFile {
//     fn map_results<NextCheck: ToBeValidated + From<Self>>(
//         self,
//     ) -> Result<NextCheck, LocalCsvError> {
//         if self.0.is_file() {
//             Ok(NextCheck::from(self))
//         } else {
//             Err(LocalCsvError::NotAFile)
//         }
//     }
// }
// impl Validate for HasCsvExtension {
//     fn map_results<NextCheck: ToBeValidated + From<Self>>(
//         self,
//     ) -> Result<NextCheck, LocalCsvError> {
//         match self.0.extension() {
//             Some(extension) if extension == "csv" => Ok(NextCheck::from(self)),
//             Some(_) => Err(LocalCsvError::NotACsv),
//             None => Err(LocalCsvError::MissingExtension),
//         }
//     }
// }

// impl From<UnverifiedPath> for IsReachableAndExists{
//     fn from(value: UnverifiedPath) -> Self {
//         Self(value.0)
//     }
// }

// impl From<IsReachableAndExists> for IsFile {
//     fn from(value: IsReachableAndExists) -> Self {
//         Self(value.0)
//     }
// }
// impl From<IsFile> for HasCsvExtension {
//     fn from(value: IsFile) -> Self {
//         Self(value.0)
//     }
// }
