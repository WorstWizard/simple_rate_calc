use crate::data::*;
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn load_database() -> Result<RecipeDB, ()> {
    let path = FileDialog::new()
        .set_location("~/")
        .add_filter("JSON", &["json"])
        .show_open_single_file()
        .unwrap();
    if let Some(path) = path {
        match File::open(path) {
            Ok(f) => {
                let buf_reader = BufReader::new(f);
                let mut deserializer = serde_json::Deserializer::from_reader(buf_reader);
                match RecipeDB::deserialize(&mut deserializer) {
                    Ok(rdb) => return Ok(rdb),
                    Err(err) => {
                        println!("{err}");
                        return Err(());
                    }
                }
            }
            Err(_) => {
                eprintln!("Cannot open file for reading");
                return Err(());
            }
        }
    }
    Err(())
}

pub fn save_database(rdb: &RecipeDB) {
    let path = FileDialog::new()
        .set_location("~/")
        .add_filter("JSON", &["json"])
        .show_save_single_file()
        .unwrap();
    if let Some(path) = path {
        match File::create(path) {
            Ok(f) => {
                let buf_writer = BufWriter::new(f);
                let mut serializer = serde_json::Serializer::new(buf_writer);
                if rdb.serialize(&mut serializer).is_err() {
                    eprintln!("Failed to serialize")
                }
            }
            Err(_) => {
                eprintln!("Cannot open file for writing")
            }
        }
    }
}
