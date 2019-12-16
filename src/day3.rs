#[allow(dead_code)]

use std::vec::Vec;
use std::ops::Range;
use std::cmp;
use std::io::{BufReader, BufRead};

use crate::util;

struct Wire {
    points: Vec<(i32, i32)>,
    extents: (i32, i32, i32, i32)
}

impl Wire {
    pub fn from_csv_reader(mut reader: csv::Reader<&[u8]>) -> Wire {
        let mut curr_pnt = (0, 0);
        let mut extents = (0, 0, 0, 0);
        let mut pnts: Vec<(i32, i32)> = vec!(curr_pnt);

        let wire_record = reader.records().next().expect("wire record expected");
        let wire_steps = wire_record.iter().next().expect("wire steps expected");

        for step in wire_steps.iter() {
            let mut char_iter = step.chars();

            let dir: char = char_iter.next().expect("no direction found");

            let step_str = char_iter.collect::<String>();

            let step_size: i32 = step_str
                .parse().unwrap();

            match dir {
                'R' => {
                    curr_pnt.0 += step_size;
                    extents.0 = cmp::max(extents.0, curr_pnt.0);
                },
                'U' => {
                    curr_pnt.1 += step_size;
                    extents.1 = cmp::max(extents.1, curr_pnt.1);
                },
                'L' => {
                    curr_pnt.0 -= step_size;
                    extents.2 = cmp::min(extents.2, curr_pnt.0);
                },
                'D' => {
                    curr_pnt.1 -= step_size;
                    extents.3 = cmp::min(extents.3, curr_pnt.1);
                },
                _ => panic!("unknown dir found '{}'", dir)
            }

            pnts.push(curr_pnt);
        }

        Wire {
            points: pnts,
            extents: extents
        }
    }

    pub fn expand(&self) -> Vec<(i32, i32)> {
        let mut pts = Vec::new();

        for start_i in 0..self.points.len() - 2 {
            let (x1, y1) = self.points[start_i];
            let (x2, y2) = self.points[start_i + 1];

            if x1 == x2 {
                // vertical line

               for y in Wire::expand_line(y1, y2) {
                    pts.push((x1, y));
               }
            } else {
                // horizontal line

                for x in Wire::expand_line(x1, x2) {
                    pts.push((x, y1));
                }
            }
        } 

        pts
    }

    fn expand_line(n1: i32, n2: i32) -> Range<i32> {
        if n1 < n2 {
            n1 + 1..n2 + 1
        } else {
            n2..n1
        }
    }
}

struct WireGrid {
    grid: Vec<Vec<bool>>,
    center: (usize, usize),
    closest: usize
}

impl WireGrid {
    pub fn from_wires(wires: &Vec<Wire>) -> WireGrid {
        let mut extremes = (0, 0, 0, 0);

        for wire in wires.iter() {
            extremes.0 = cmp::max(extremes.0, wire.extents.0);
            extremes.1 = cmp::max(extremes.1, wire.extents.1);
            extremes.2 = cmp::min(extremes.2, wire.extents.2);
            extremes.3 = cmp::min(extremes.3, wire.extents.3);
        }

        let size = ((extremes.0 + extremes.2.abs() + 1) as usize, 
                   (extremes.1 + extremes.3.abs() + 1) as usize);

        let grid: Vec<Vec<bool>> = vec![vec![false; size.1]; size.0];

        let center = (extremes.2.abs() as usize,
                      extremes.3.abs() as usize);

        let (a, b) = size;
        let (x, y) = center;
        println!("size: ({}, {})\ncenter: ({}, {})", a, b, x, y);

        WireGrid {
            grid: grid,
            center: center,
            closest: 0
        }        
    }
    
    pub fn add_wire(&mut self, wire: &Wire) {
        let pts = wire.expand();

        for pt in pts {
            let (x, y) = self.to_grid(&pt);

            if self.grid[x][y] && (x != self.center.0 && y != self.center.1) {
                self.closest = cmp::min(self.closest,
                                        self.distance_to_center(&(x, y)));
            }

            self.grid[x][y] = true;
        }
    }

    fn to_grid(&self, point: &(i32, i32)) -> (usize, usize) {
        ((point.0 - self.center.0 as i32) as usize,
         (point.1 - self.center.1 as i32) as usize)
    }

    fn distance_to_center(&self, grid_point: &(usize, usize)) -> usize {
        (grid_point.0 - self.center.0) + (grid_point.1 - self.center.1)
    }
}

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

    for wire in wires {
        wire_grid.add_wire(&wire);
    }

    println!("part 1: {}", wire_grid.closest);
}