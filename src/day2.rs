use crate::int_code;
use crate::util;

enum Status {
    Ok,
    Halt,
    Invalid
}

fn do_op(opcodes: &mut Vec<i32>, ind: usize) -> Status {
    let op = opcodes[ind];

    let left = opcodes[ind + 1] as usize;
    let right = opcodes[ind + 2] as usize;
    let dest = opcodes[ind + 3] as usize;

    let new = match op {
        99 => return Status::Halt,
        1 => opcodes[left] + opcodes[right],
        2 => opcodes[left] * opcodes[right],
        _ => return Status::Invalid,
    };

    opcodes[dest] = new;

    Status::Ok
}

pub fn solve() {
    let file = util::get_input(2); 

    let int_code = int_code::IntCode::new(&file);

    println!("value at first index: {}", 0);
}