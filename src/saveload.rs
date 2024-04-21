use std::io::{BufWriter, BufReader};
use std::fs::File;
use serde::{Deserialize, Serialize};
use native_dialog::FileDialog;
use crate::data::*;

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
                    Ok(rdb) => { return Ok(rdb) },
                    Err(err) => { println!("{err}"); return Err(()) }
                }
            },
            Err(_) => { eprintln!("Cannot open file for reading"); return Err(()) }
        }
    }
    println!("didn't find file?");
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
                match rdb.serialize(&mut serializer) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            },
            Err(_) => { eprintln!("Cannot open file for writing") }
        }
    }
}