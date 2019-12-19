#[allow(dead_code)]

use std::fs::File;
use std::vec::Vec;

use csv::Reader;

pub struct IntCode {
    memory: Vec<i32>,
    current_state: Vec<i32>,
    instr_ptr: usize
}

impl IntCode {
    pub fn new(mut input: Reader<File>) -> IntCode {
        let mut opcodes: Vec<i32> = Vec::new(); 

        for record in input.records() {
            for code in record.unwrap().iter() {
                opcodes.push(code.parse().unwrap());
            }
        }

        IntCode {
            memory: opcodes,
            current_state: Vec::new(),
            instr_ptr: 0
        }
    }

    pub fn process_inputs(&mut self, noun: i32, verb: i32) -> i32 {
        self.current_state = self.memory.to_vec();
        self.current_state[1] = noun;
        self.current_state[2] = verb;
        self.instr_ptr = 0; 

        loop {
            let instr = self.current_state[self.instr_ptr];

            match instr {
                1 => self.math_instr(|n1, n2| n1 + n2),
                2 => self.math_instr(|n1, n2| n1 * n2),
                99 => break,
                _ => {
                    println!("invalid instr reached, {}", instr);
                    break;
                }
            };
        }

        self.current_state[0]
    }

    fn math_instr<F>(&mut self, op: F) where
        F: Fn(i32, i32) -> i32 {
        let c_state = &self.current_state;
        let i_ptr = self.instr_ptr;

        let left = c_state[c_state[i_ptr + 1] as usize];
        let right = c_state[c_state[i_ptr + 2] as usize];
        let dest = c_state[i_ptr + 3] as usize; 
        
        self.current_state[dest] = op(left, right);
        self.instr_ptr += 4;
    }
}