mod int_code;

use int_code::IntCode;
use crate::util;

pub fn solve() {
    let input = util::get_input(5);

    let mut machine = IntCode::new(input);

    machine.run_diagnostic();
}