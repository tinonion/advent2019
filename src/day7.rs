mod int_code;
mod permutation;

use std::cmp;

use int_code::IntCode;
use crate::util;

pub fn solve() {
    let input = util::get_input(7);
    let mut machine = IntCode::new(input);

    let phases = vec!(0, 1, 2, 3, 4);
    let mut max_power = 0;

    for phase_sequence in permutation::find_permutations(phases) {
        let power = machine.process_phase_sequence(phase_sequence);

        max_power = cmp::max(power, max_power);        
    }

    println!("part 1: {}", max_power);
}