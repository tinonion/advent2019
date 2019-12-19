use std::vec::Vec;
use std::ops::Range;
use std::cmp;

#[derive(PartialEq, Debug)]
pub struct Wire {
    pub points: Vec<(i32, i32)>,
    pub extents: (i32, i32, i32, i32)
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

        for start_i in 0..self.points.len() - 1 {
            let (x1, y1) = self.points[start_i];
            let (x2, y2) = self.points[start_i + 1];

            if x1 == x2 {
                // vertical line
                if y1 < y2 {
                    // going up
                    for y in y1 + 1..y2 + 1 {
                        pts.push((x1, y));
                    }
                } else {
                    // going down
                    for y in (y2..y1).rev() {
                        pts.push((x1, y));
                    }
                }

            } else {
                // horizontal line
                if x1 < x2 {
                    // going right
                    for x in x1 + 1..x2 + 1 {
                        pts.push((x, y1));
                    }
                } else {
                    // going left
                    for x in (x2..x1).rev() {
                        pts.push((x, y1));
                    }
                }
            }
        } 

        pts
    }

    fn eq(&self, other: &Self) -> bool {
        let extents_eq = self.extents == other.extents;
        let points_eq = self.points == other.points;

        extents_eq && points_eq
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::ReaderBuilder;

    #[test]
    fn create_wire() {
        let csv_input = "R8,U5,L5,D10";

        let reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_input.as_bytes());

        let wire = Wire::from_csv_reader(reader);

        let extents = (8, 5, 0, -5);
        let points = vec!((0, 0), (8, 0), (8, 5), (3, 5), (3, -5));
        let expected = Wire { extents: extents, points: points };

        assert_eq!(wire, expected);
    }

    #[test]
    fn expand_wire_simple() {
        let points = vec!((0, 0), (3, 0), (5, 0));
        let extents = (10, 10, 0, 0);

        let wire = Wire {
            points: points,
            extents: extents
        };

        let expanded = vec!((1, 0), (2, 0), (3, 0), (4, 0), (5, 0));

        assert_eq!(wire.expand(), expanded);
    }

    #[test]
    fn expand_wire() {
        let points = vec!((0, 0), (3, 0), (3, -3), (-2, -3));
        let extents = (3, 0, -2, -3);

        let wire = Wire {
            points: points,
            extents: extents
        };

        let expanded = vec!((1, 0), (2, 0), (3, 0),
            (3, -1), (3, -2), (3, -3),
            (2, -3), (1, -3), (0, -3), (-1, -3), (-2, -3));

        assert_eq!(wire.expand(), expanded);
    }
}