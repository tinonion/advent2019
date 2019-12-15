use std::fs::File;
use std::path::PathBuf;

pub fn get_input(day: u32) -> File {
    let day: String = day.to_string() + ".txt";

    let mut path: PathBuf = PathBuf::from("./data");
    path.push(day);

    File::open(&path).expect(&format!("unable to open input file: {:?}", &path))
}