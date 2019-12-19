#[allow(dead_code)]

pub mod wire;
pub mod wire_grid;

use std::io::{BufReader, BufRead};

use wire::Wire;
use wire_grid::WireGrid;
use crate::util;

pub fn solve() {
    let file = util::get_input(3);

    let line_reader = BufReader::new(file);

    let mut wires: Vec<Wire> = Vec::new();
    for line in line_reader.lines() {
        let wire = line.unwrap();

        let wire_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(wire.as_bytes());

        let wire = Wire::from_csv_reader(wire_reader);        
        wires.push(wire);
    }

    let mut wire_grid = WireGrid::from_wires(&wires);

    for (id, wire) in wires.iter().enumerate() {
        wire_grid.add_wire(&wire, id as i32);
    }

    println!("part 1: {}", wire_grid.closest);
    println!("part 2: {}", wire_grid.shortest);
}