// mod categorical_variants;

use std::{error::Error, fs::File, io::Write, ops::Add, path::PathBuf, sync::Arc};

use clap::Parser;
use file_utility::{self, arg_parsing::Cli};
use polars::prelude::*;
fn main() -> Result<(), Box<dyn Error>> {
    let Cli {
        data_path,
        feature_desc_path: _feature_desc_path,
        output_path,
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

    let mut type_gen = TypeGenerator(Vec::new());
    let info = df
        .get_columns()
        .iter()
        .map(|col| {
            let column_unique_values = col.unique().unwrap();
            let name = col.name().as_str();
            let col_variants = column_unique_values
                .str()
                .unwrap()
                .into_iter()
                .flatten()
                .collect::<Vec<&str>>();

            type_gen.generate_type(name, col_variants)
        })
        .fold(
            GenerationInfo::new(),
            |final_info, info| final_info + info,
        );
    let buffer = type_gen.generate_string();
    TypeGenerator::write(
        output_path.expect("Must give the output_path with -o arg followed by path"),
        &buffer,
        false,
    );
    info.report();
    Ok(())
}

enum EnumValidationError<'a> {
    Name(&'a str),
    Variant(Vec<&'a str>),
    NameAndVariants {
        name: &'a str,
        variants: Vec<&'a str>,
    },
}

struct EnumDef {
    name: String,
    variants: Vec<String>,
}
enum GenType {
    TypeEnum,
    TypeStruct,
}

struct TypeGenerator(Vec<EnumDef>);
impl TypeGenerator {
    fn generate_string(self) -> String{
        let prefix = String::from("#![allow(unused)]\n\nuse serde::Deserialize;");
        let mut enums = String::new();
        let mut deserializer_struct = String::new();
        deserializer_struct.push_str("#[derive(Debug,Deserialize)]\n");
        deserializer_struct.push_str("struct SerdeCsvDeserializer{");
        for endef in self.0{
            enums.push_str("#[derive(Debug,Deserialize)]\n");
            enums.push_str(&Self::gen_enum(&endef));
            enums.push('\n');
            
            deserializer_struct.push_str(format!("{} : {},",endef.name.to_lowercase(), endef.name).as_str());
        }
        deserializer_struct.push('}');
        prefix + &enums + &deserializer_struct
    }
    fn add_enum(&mut self, enum_def: EnumDef){
        self.0.push(enum_def);
    }
    /// It first checks if name and variants are valid, if they are, enum is generated.
    /// Otherwise try to make name and variants valid, wrapper struct if generated if either one of them is
    /// not valid or Some((name,variants)) if the transformation produced a valid enum
    fn generate_type(&mut self, name: &str, variants: Vec<&str>) -> GenerationInfo {
        let match_gen_type = |gen_type: GenType| match gen_type {
            GenType::TypeEnum => GenerationInfo::new().increase_recovered().append_name(name),
            GenType::TypeStruct => GenerationInfo::new().increase_wrapper(),
        };

        match Self::validate_enum(name, variants.clone()) {
            Ok(enum_def) => {
                self.0.push(enum_def);

                GenerationInfo::new().increase_enum()
            }
            Err(err) => match err {
                EnumValidationError::Name(name) => {
                    let name = Self::fix_variant_or_name(name);

                    match_gen_type(self.gen_enum_otherwise_struct(&name, variants))
                }
                EnumValidationError::Variant(variants) => {
                    let variants = variants
                        .into_iter()
                        .map(Self::fix_variant_or_name)
                        .collect::<Vec<String>>();

                    match_gen_type(self.gen_enum_otherwise_struct(
                        name,
                        variants.iter().map(|x| x.as_str()).collect(),
                    ))
                }
                EnumValidationError::NameAndVariants { name, variants } => {
                    let name = Self::fix_variant_or_name(name);
                    let variants = variants
                        .into_iter()
                        .map(Self::fix_variant_or_name)
                        .collect::<Vec<_>>();

                    match_gen_type(self.gen_enum_otherwise_struct(
                        &name,
                        variants.iter().map(|x| x.as_str()).collect(),
                    ))
                }
            },
        }
    }
    fn gen_enum_otherwise_struct(&mut self, name: &str, variants: Vec<&str>) -> GenType {
        if let Ok(enum_def) = Self::validate_enum(name, variants.clone()) {
            self.add_enum(enum_def);
            GenType::TypeEnum
        } else {
            GenType::TypeStruct
        }
    }
    fn validate_enum<'a>(
        name: &'a str,
        variants: Vec<&'a str>,
    ) -> Result<EnumDef, EnumValidationError<'a>> {
        let name_is_valid = Self::is_valid_enum_variant_name(name);
        let mut invalid_variants: Vec<&'a str> = vec![];

        variants.iter().for_each(|variant| {
            if !Self::is_valid_enum_variant_name(variant) {
                invalid_variants.push(variant);
            }
        });

        let invalid_variants_is_empty = invalid_variants.is_empty();
        if invalid_variants_is_empty && name_is_valid {
            Ok(EnumDef {
                name: name.to_string(),
                variants: variants.iter().map(|x| x.to_string()).collect(),
            })
        } else if invalid_variants_is_empty && !name_is_valid {
            Err(EnumValidationError::Name(name))
        } else if !invalid_variants_is_empty && name_is_valid {
            Err(EnumValidationError::Variant(invalid_variants))
        } else {
            Err(EnumValidationError::NameAndVariants {
                name,
                variants: invalid_variants,
            })
        }
    }
    /// Accept either a name or a variant of the enum to fix
    pub fn fix_variant_or_name(variant: &str) -> String {
        variant
            .trim()
            .chars()
            .map(|ch| match ch {
                // Common Punctuation
                '.' => "Point".to_string(),
                ',' => "Comma".to_string(),
                ':' => "Colon".to_string(),
                ';' => "Semi".to_string(),
                '_' => "Underscore".to_string(),

                // Math & Logic
                '+' => "Plus".to_string(),
                '-' => "Minus".to_string(),
                '*' => "Star".to_string(),
                '/' => "Slash".to_string(),
                '=' => "Equals".to_string(),
                '%' => "Percent".to_string(),
                '<' => "Lt".to_string(), // Less Than
                '>' => "Gt".to_string(), // Greater Than

                // Wrapper Symbols
                '(' => "OpenParen".to_string(),
                ')' => "CloseParen".to_string(),
                '[' => "OpenBracket".to_string(),
                ']' => "CloseBracket".to_string(),
                '{' => "OpenBrace".to_string(),
                '}' => "CloseBrace".to_string(),

                // Special / Web
                '@' => "At".to_string(),
                '#' => "Hash".to_string(),
                '$' => "Dollar".to_string(),
                '&' => "And".to_string(),
                '|' => "Pipe".to_string(),
                '!' => "Bang".to_string(),
                '?' => "Question".to_string(),
                '~' => "Tilde".to_string(),
                ' ' => "Space".to_string(), // Optional: usually stripped in Enums

                // Quotes
                '"' => "Quote".to_string(),
                '\'' => "Tick".to_string(),
                '`' => "Backtick".to_string(),
                '\\' => "Backslash".to_string(),

                // Mapping dei Numeri
                '0' => "Zero".to_string(),
                '1' => "One".to_string(),
                '2' => "Two".to_string(),
                '3' => "Three".to_string(),
                '4' => "Four".to_string(),
                '5' => "Five".to_string(),
                '6' => "Six".to_string(),
                '7' => "Seven".to_string(),
                '8' => "Eight".to_string(),
                '9' => "Nine".to_string(),

                // Default: Keep alphanumeric characters as they are
                c => c.to_string(),
            })
            .collect() // Joins the parts into a single String
    }
    /// Can be used to validate either name or a variant of an enum
    fn is_valid_enum_variant_name(variant: &str) -> bool {
        let mut chars = variant.chars();

        let first_char = chars.next().unwrap();
        if first_char.is_numeric() {
            return false;
        }
        !chars.any(|ch| !ch.is_alphanumeric())
    }
    fn gen_enum(enum_def: &EnumDef) -> String {
        let variants = enum_def
            .variants
            .iter()
            .map(|x| format!("{x},"))
            .collect::<String>();
        let name = &enum_def.name;
        format!("enum {name}{{{variants}}}\n")
    }
    /// `file_name`: name of the rust file there no extension validation.\
    /// `buffer`: should contains all the enums and struct to represent data of the csv.\
    /// `dry_run`: if true just print the buffer and do not write anything.
    fn write(output_path: impl Into<PathBuf>, buffer: &str, dry_run: bool) {
        if !dry_run {
            let path: PathBuf = output_path.into();

            if path.exists() {
                println!("File aready exists")
            } else {
                let mut file = File::create(path).unwrap();
                file.write_all(buffer.as_bytes()).unwrap();
            }
        } else {
            println!("{buffer}");
        }
    }

    /// `name` is assumed valid otherwise cannot write struct enum
    fn gen_wrapper_struct(name: &str, variants: Vec<&str>) -> String {
        let mut variants_len = 0;
        let variants = variants
            .into_iter()
            .map(|var_name| {
                variants_len += 1;
                if variants_len > 100 {
                    panic!("More than 100 variants found for `{name}` column")
                }
                format!("\"{var_name}\",")
            })
            .collect::<String>();
        let name_upper = name.to_uppercase();
        format!(
            "struct {name}([&'static str;{variants_len}]);\nconst {name_upper}: {name} = {name}([{variants}]);\n",
        )
    }
}

/// I should fix recovered increase
/// as of now it just increase enum_counter
struct GenerationInfo {
    enum_counter: u32,
    recovered_invalid_enums: u32,
    recovered_names: Vec<String>,
    wrapper_struct_counter: u32,
}

impl GenerationInfo {
    fn new() -> Self {
        let (enum_counter, recovered_invalid_enums, recovered_names, wrapper_struct_counter) =
            (0, 0, Vec::new(), 0);
        Self {
            enum_counter,
            recovered_invalid_enums,
            recovered_names,
            wrapper_struct_counter,
        }
    }
    fn increase_enum(mut self) -> Self {
        self.enum_counter += 1;
        self
    }
    fn increase_recovered(mut self) -> Self {
        self.recovered_invalid_enums += 1;
        self
    }
    fn increase_wrapper(mut self) -> Self {
        self.wrapper_struct_counter += 1;
        self
    }
    fn append_name(mut self, name: impl Into<String>) -> Self {
        self.recovered_names.push(name.into());
        self
    }
    fn report(&self) {
        println!(
            "--~ Generation Report\n number of enums correctly generated: {}",
            self.enum_counter
        );
        println!(
            " number of enums recovered: {}",
            self.recovered_invalid_enums
        );
        println!(" recovered columns: {:?}", self.recovered_names);
    }
}
impl Add for GenerationInfo {
    type Output = GenerationInfo;

    fn add(self, rhs: Self) -> Self::Output {
        let recovered_names: Vec<String> = self
            .recovered_names
            .into_iter()
            .chain(rhs.recovered_names)
            .collect();
        GenerationInfo {
            enum_counter: self.enum_counter + rhs.enum_counter,
            recovered_invalid_enums: self.recovered_invalid_enums + rhs.recovered_invalid_enums,
            wrapper_struct_counter: self.wrapper_struct_counter + rhs.wrapper_struct_counter,
            recovered_names,
        }
    }
}
