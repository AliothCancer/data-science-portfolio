mod categorical_variants;

use std::{error::Error, fs::File, io::Write, path::Path, sync::Arc};

use clap::Parser;
use file_utility::{self, arg_parsing::Cli};
use itertools::Itertools;
use polars::prelude::*;
fn main() -> Result<(), Box<dyn Error>> {
    let Cli {
        data_path,
        feature_desc_path: _feature_desc_path,
    } = Cli::parse();
    let df = LazyCsvReader::new(PlPath::Local(Arc::from(data_path)))
        .with_null_values(Some(polars::prelude::NullValues::AllColumns(vec![
            "NA".into(),
        ])))
        .with_has_header(true)
        .finish()?;

    let df = df
        .select([
            Selector::ByDType(DataTypeSelector::AnyOf(Arc::from([DataType::String]))).as_expr(),
        ])
        .collect()?;
    // dbg!(df);
    let buffer = String::from("#![allow(unused)]\n\n") + &df
        .get_columns()
        .into_iter()
        .map(|col| {
            let column = col.unique().unwrap();
            let name = col.name().as_str();
            let col = column.str().unwrap().to_owned();

            TypeGenerator::generate_type(name, col) + "\n"
        })
        .collect::<String>();

    TypeGenerator::write("categorical_variants.rs", &buffer, false);
    Ok(())
}

const ROOT: &str = "/home/giulio/Scrivania/rust_data_science_env/generate_types_from_csv/src";
// enum VariantError {
//     NoUniqueValues,
// }
struct TypeGenerator;
impl TypeGenerator {
    /// Can be used to validate either name or variants of an enum
    fn is_valid_enum_variant_name(variant: &str) -> bool {
        let mut chars = variant.chars();

        let first_char = chars.next().unwrap();
        if first_char.is_numeric() {
            return false;
        }
        if chars.any(|ch| !ch.is_alphanumeric()) {
            return false;
        } else {
            true
        }
    }
    fn generate_type<'a>(name: &'a str, variants: ChunkedArray<StringType>) -> String {
        let name_is_valid = TypeGenerator::is_valid_enum_variant_name(name);
        let variants_are_valid = variants
            .iter()
            .filter_map(|x| x)
            .all(TypeGenerator::is_valid_enum_variant_name);
        let variants = variants.into_iter().filter_map(|x| x);

        if name_is_valid && variants_are_valid {
            Self::gen_enum(name, variants)
        } else if name_is_valid {
            Self::gen_wrapper_struct(name, variants)
        } else {
            panic!("At least the names of the column should be valid as rust struct names")
        }
    }
    fn gen_enum<'a>(name: &'a str, variants: impl Iterator<Item = &'a str>) -> String {
        let variants = variants.map(|x| format!("{x},")).collect::<String>();
        format!("enum {name}{{{variants}}}\n")
    }
    /// `file_name`: name of the rust file there no extension validation.\
    /// `buffer`: should contains all the enums and struct to represent data of the csv.\
    /// `dry_run`: if true just print the buffer and do not write anything. 
    fn write(file_name: &str, buffer: &str, dry_run: bool) {
        if !dry_run {
            let path = Path::new(ROOT).join(file_name);
            
            if path.exists(){
                println!("File aready exists")
            }else {
                let mut file = File::create(path).unwrap();
                file.write_all(buffer.as_bytes()).unwrap();
            }
        }else {
            println!("{buffer}");
        }
    }

    /// `name` is assumed valid otherwise cannot write struct enum
    fn gen_wrapper_struct<'a>(name: &str, variants: impl Iterator<Item = &'a str>) -> String {
        let mut variants_len = 0;
        let variants = variants
            .map(|var_name| {
                variants_len += 1;
                if variants_len > 100{
                    panic!("More than 100 variants found for `{name}` column")
                }
                format!("\"{var_name}\",")
            })
            .collect::<String>();
        let name_upper = name.to_uppercase();
        format!("struct {name}([&'static str;{variants_len}]);\nconst {name_upper}: {name} = {name}([{variants}]);\n",)
    }
}
