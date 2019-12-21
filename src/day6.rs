mod orbit_system;

use std::io::BufRead;
use regex::Regex;

use crate::util;
use orbit_system::OrbitSystem;

pub fn solve() {
    let input = util::get_input_line_reader(6);
    let mut orbit_system = OrbitSystem::new();

    let orbit_pat = Regex::new(r"(\w+)\)(\w+)").unwrap();

    for orbit in input.lines() {
        let orbit = orbit.unwrap();

        let caps = orbit_pat.captures(&orbit).unwrap();

        let primary_body = caps.get(1).unwrap().as_str();
        let satellite = caps.get(2).unwrap().as_str();
        
        orbit_system.add_relation(primary_body, satellite);
    }

    println!("part 1: {}", orbit_system.find_distance());
    println!("part 2: {}", orbit_system.path_to_santa());
}