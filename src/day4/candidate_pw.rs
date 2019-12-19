use std::vec::Vec;

use super::to_ascii;

pub struct CandidatePw {
    pub current_pw: Vec<char>
}

impl CandidatePw {
    pub fn new() -> CandidatePw {
        CandidatePw {
            current_pw: Vec::new()
        }
    }

    pub fn from_pw(pw: &str) -> CandidatePw {
        CandidatePw {
            current_pw: to_ascii(pw)
        }
    }

    pub fn get_pw_str(&self) -> String {
        let mut new_str = String::from("");

        for c in self.current_pw.iter() {
            new_str.push(*c);
        }

        new_str
    }

    pub fn validate(&self) -> bool {
        let pw = &self.current_pw;

        if pw.len() <= 2 {
            panic!("Don't try to validate a password that is 2 chars or less");
        }

        let mut left = 0;
        let mut right = 0;
        while left < pw.len() - 1 {
            if left > right {
                panic!("left is greater than right {}, {}", left, right);
            }

            if right == pw.len() {
                return right - left == 2;
            }

            if pw[left] == pw[right] {
                right += 1;

            } else if right - left == 2 {
                return true;

            } else {
                left = right;
            }
        }

        return false
    }

    pub fn new_cand(&self, next_c: char) -> CandidatePw {
        let mut pw_cp = self.current_pw.to_vec();
        pw_cp.push(next_c);

        CandidatePw {
            current_pw: pw_cp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let cand = CandidatePw::from_pw("666999");

        assert!(!cand.validate());
    }
}