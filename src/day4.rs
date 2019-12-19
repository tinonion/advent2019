mod password_finder;
mod candidate_pw;

use password_finder::PasswordFinder;

pub fn to_ascii(s: &str) -> Vec<char> {
    s.chars().map(|c| c as char).collect()
}

pub fn solve() {
    let min = 307237;
    let max = 769058;

    let mut finder = PasswordFinder::new(min, max);

    println!("unique p/w count: {}", finder.find_unique()); 
}