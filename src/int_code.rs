use std::fs::File;
use std::vec::Vec;

use csv::{ReaderBuilder};

pub struct IntCode {
    memory: Vec<i32>,
    current_state: Vec<i32>,
    instr_ptr: usize,
    complete: bool
}

enum Params {
    Add(usize, usize, usize),
    Mult(usize, usize, usize)
}

impl IntCode {
    pub fn new(input: &File) -> IntCode {
        // build csv reader, then parse in fields
        let mut reader = ReaderBuilder::new()
            .has_headers(false)        
            .from_reader(input);

        let mut opcodes: Vec<i32> = Vec::new(); 

        for record in reader.records() {
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

    pub fn process_inputs(&self, noun: i32, verb: i32) {
        self.current_state = self.memory.to_vec();
        self.instr_ptr = 0; 

        loop {
            let instr = self.current_state[self.instr_ptr];

            match instr {
                1 => self.add_instr(),
                2 => self.mult_instr(),
                99 => break,
                _ => {
                    println!("invalid instr reached, {}", instr);
                    break;
                }
            };
        }
    }

    fn math_instr(op: fn(i32) -> i32)

    fn add_instr(&self) {
        let c_state = &self.current_state;
        let i_ptr = self.instr_ptr;

        let left = c_state[i_ptr + 1];
        let right = c_state[i_ptr + 2];
        let dest = c_state[i_ptr + 3] as usize; 

        self.current_state[dest] = left + right;
    }

    fn mult_instr(&self) {
        let c_state = &self.current_state;
        let i_ptr = self.instr_ptr;

        let left = c_state[i_ptr + 1];
        let right = c_state[i_ptr + 2];
        let dest = c_state[i_ptr + 3] as usize; 

        self.current_state[dest] = left * right;
    }

    fn check_eop(&self, instr_len: usize) -> Params {

        Params::Ok
    }

    fn extract_params(&self, param_cnt: usize) -> Vec<i32> {
        let params: Vec<i32> = Vec::new();

        let i_ptr = self.instr_ptr;

        for param_i in i_ptr..(i_ptr + param_cnt) {
            params.push(self.current_state[param_i]);
            self.instr_ptr += 1;
        }

        params
    }
}