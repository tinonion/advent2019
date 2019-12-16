#[allow(dead_code)]

use std::fs::File;
use std::path::PathBuf;

use csv::{Reader, ReaderBuilder};

pub fn get_input(day: u32) -> File {
    let day: String = day.to_string() + ".txt";

    let mut path: PathBuf = PathBuf::from("./data");
    path.push(day);

    File::open(&path).expect(&format!("unable to open input file: {:?}", &path))
}

pub fn to_csv_reader(file: File) -> Reader<File> {
    ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file)
}