#[allow(dead_code)]

use std::fs::File;
use std::path::PathBuf;

fn find_data_path(day: u32, dir: &str, ext: &str) -> String {
    let day: String = day.to_string() + ext;

    let mut path: PathBuf = PathBuf::from(dir);
    path.push(day);

    String::from(path.to_str().expect("problem converting data path to string"))
}

pub fn get_input(day: u32) -> File {
    let path = find_data_path(day, "./data", ".txt");

    File::open(&path).expect(&format!("unable to open input file: {:?}", &path))
}

pub fn get_test_input(day: u32) -> File {
    let path = find_data_path(day, "./data", ".test");

    File::open(&path).expect(&format!("unable to open input file: {:?}", &path))
}