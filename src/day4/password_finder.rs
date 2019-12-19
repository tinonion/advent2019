#[allow(dead_code)]

use std::vec::Vec;
use super::candidate_pw::CandidatePw;
use super::to_ascii;

const PW_LEN: usize = 6;

pub struct PasswordFinder {
    min: Vec<char>,
    max: Vec<char>,
    total_unique: i32
}

impl PasswordFinder {
    pub fn new(min: i32, max: i32) -> PasswordFinder {
        PasswordFinder {
            min: to_ascii(&min.to_string()),
            max: to_ascii(&max.to_string()),
            total_unique: 0
        }                       
    }

    pub fn find_unique(&mut self) -> i32 {
        let mut start_cand = CandidatePw::new();

        self.delve(&mut start_cand);

        self.total_unique
    }

    fn delve(&mut self, cand: &mut CandidatePw) {
        if !self.in_range(&cand.current_pw) {
            // out of range
            return;
        }

        let len = cand.current_pw.len();

        if len >= PW_LEN {
            // reached end of candidate, still need to check consecutive
            if cand.validate() {
                // found successful candidate!
                self.total_unique += 1;
            }

            return;
        }

        let last_digit = if len == 0 {
            0
        } else {
            cand.current_pw[len - 1] as u8
        };

        // find number of possible increasing digits to add to candidate
        let possible = ('9' as u8) - last_digit;

        for p in 0..possible + 1 {
            let new_c = (last_digit + p) as char;
            let mut new_cand = cand.new_cand(new_c);

            self.delve(&mut new_cand);
        }
    }

    fn in_range(&self, pw: &Vec<char>) -> bool {
        let mut over_min = false;
        let mut under_max = false;

       for i in 0..pw.len() {
           if pw[i] > self.min[i] {
               over_min = true;
           }

           if pw[i] < self.max[i] {
               under_max = true;
           }

           if (!under_max && pw[i] > self.max[i]) ||
                    (!over_min && pw[i] < self.min[i]) {
               return false;
           }
       } 

       return true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_range() {
        let finder = PasswordFinder::new(344566, 667899);

        let f = |s: &str| { finder.in_range(&to_ascii(s))};

        assert!(!finder.in_range(&to_ascii("344506")));
        assert!(finder.in_range(&to_ascii("344566")));
        assert!(finder.in_range(&to_ascii("346666")));
        assert!(finder.in_range(&to_ascii("667899")));
        assert!(!finder.in_range(&to_ascii("667999")));

        assert!(f(""));
        assert!(f("344"));
        assert!(f("667"));
        assert!(!f("677"));
        assert!(!f("334"));
        assert!(!f("2"));
    }
}