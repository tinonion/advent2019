use crate::int_code;
use crate::util;

pub fn solve() {
    let file = util::get_input(2); 

    let mut int_code = int_code::IntCode::new(&file);

    println!("part 1: {}", int_code.process_inputs(12, 2));

    let desired = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            if desired == int_code.process_inputs(noun, verb) {
                println!("part 2: {}", 100 * noun + verb);
                break;
            }
        }
    }
}