use std::vec::Vec;
use std::cmp;

use super::wire::Wire;

fn unsigned_diff(n1: usize, n2: usize) -> usize {
    if n1 < n2 {
        n2 - n1
    } else {
        n1 - n2
    }
}

#[derive(PartialEq, Debug)]
pub struct WireGrid {
    grid: Vec<Vec<(i32, usize)>>,
    center: (usize, usize),
    pub closest: usize,
    pub shortest: usize
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

        let grid: Vec<Vec<(i32, usize)>> = vec![vec![(-1, 0); size.1]; size.0];

        let center = (extremes.2.abs() as usize,
                      extremes.3.abs() as usize);

        WireGrid {
            grid: grid,
            center: center,
            closest: std::usize::MAX,
            shortest: std::usize::MAX
        }        
    }
    
    pub fn add_wire(&mut self, wire: &Wire, id: i32) {
        let pts = wire.expand();

        for (i, pt) in pts.iter().enumerate() {
            let (x, y) = self.to_grid(&pt);

            let grid_id = self.grid[x][y].0;
            let grid_time = self.grid[x][y].1;

            if grid_id != -1 && grid_id != id {
                self.closest = cmp::min(self.closest,
                                        self.distance_to_center(&(x, y)));

                self.shortest = cmp::min(self.shortest,
                                         grid_time + i + 1);
            }

            self.grid[x][y] = (id, i + 1);
        }
    }

    fn to_grid(&self, point: &(i32, i32)) -> (usize, usize) {
        ((point.0 + self.center.0 as i32) as usize,
         (point.1 + self.center.1 as i32) as usize)
    }

    fn distance_to_center(&self, grid_point: &(usize, usize)) -> usize {
        unsigned_diff(self.center.0, grid_point.0) + 
            unsigned_diff(self.center.1, grid_point.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_grid() {
        let grid = WireGrid {
            grid: Vec::new(),
            center: (5, 10),
            closest: 0,
            shortest: 0
        };

        let grid_pt = (7, 14);

        assert_eq!(grid.to_grid(&(2, 4)), grid_pt);
    }

    #[test]
    fn distance_to_center() {
        let grid = WireGrid {
            grid: Vec::new(),
            center: (5, 10),
            closest: 0,
            shortest: 0
        };

        assert_eq!(grid.distance_to_center(&(2, 4)), 9);
        assert_eq!(grid.distance_to_center(&(7, 12)), 4);
    }
}
