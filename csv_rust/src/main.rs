
mod csv_types;
use csv_types::*;
fn main() {
    let path = "/home/giulio/Scrivania/rust_data_science_env/py/only_str.csv";
    let mut reader = csv::Reader::from_path(path).unwrap();

    let df = reader.deserialize::<SerdeCsvDeserializer>().collect::<Vec<_>>();

    dbg!(&df[1]);
}
