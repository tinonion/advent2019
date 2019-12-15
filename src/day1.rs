use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::util;

fn fuel_cost(mass: i32, fuel: i32) -> i32 {
    let new_fuel = (mass / 3) as i32 - 2;

    if new_fuel <= 0 {
        return fuel;
    }

    return fuel_cost(new_fuel, fuel + new_fuel);
}

pub fn solve() {
    let input: File = util::get_input(1);
    let input = BufReader::new(input);

    let mut total_fuel: i32 = 0;
    
    for line in input.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();

        total_fuel += fuel_cost(mass, 0);
    }

    println!("total fuel: {}", total_fuel);
}