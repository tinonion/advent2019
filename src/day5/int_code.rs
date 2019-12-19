#[allow(dead_code)]

use std::fs::File;
use std::vec::Vec;

use csv::ReaderBuilder;

struct Operation {
    pub opcode: i32,
    pub parameters: [i32; 3]
}

pub struct IntCode {
    memory: Vec<i32>,
    current_state: Vec<i32>,
    instr_ptr: usize
}

impl IntCode {
    pub fn new(input: File) -> IntCode {
        let mut input = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(input);

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

    pub fn run_diagnostic(&mut self) {
        self.current_state = self.memory.to_vec();
        self.instr_ptr = 0; 

        loop {
            let instr = self.current_state[self.instr_ptr];
            let instr = IntCode::extract_op(instr);

            match instr {
                1 => self.math_instr(|n1, n2| n1 + n2),
                2 => self.math_instr(|n1, n2| n1 * n2),
                3 => self.input_instr(),
                4 => self.output_instr(),
                5 => self.jump_instr(false),
                6 => self.jump_instr(true),
                7 => self.cmp_instr(|n1, n2| n1 < n2),
                8 => self.cmp_instr(|n1, n2| n1 == n2),
                99 => break,
                _ => {
                    println!("invalid instr reached, {}, at {}", instr, self.instr_ptr);
                    break;
                }
            };
        }
    }

    fn jump_instr(&mut self, jump_if_zero: bool) {
        let mut operation = self.collect_operation(2);
        self.mode_switch(&mut operation, 2);

        let [to_cmp, new_ptr, _] = operation.parameters;

        let is_zero = to_cmp == 0;

        if (!jump_if_zero && !is_zero) || (jump_if_zero && is_zero) {
            self.instr_ptr = new_ptr as usize;
        }
    }

    fn cmp_instr<F>(&mut self, pred: F) where
        F: Fn(i32, i32) -> bool {

        let mut operation = self.collect_operation(3); 
        self.mode_switch(&mut operation, 2);

        let [first, second, dest] = operation.parameters;

        let result = if pred(first, second) { 1 } else { 0 };

        self.current_state[dest as usize] = result;
    }

    fn math_instr<F>(&mut self, op: F) where
        F: Fn(i32, i32) -> i32 {

        let mut operation = self.collect_operation(3);

        // only mode switch first two parameters
        self.mode_switch(&mut operation, 2);
        let [left, right, dest] = operation.parameters;

        self.current_state[dest as usize] = op(left, right);
    }

    fn input_instr(&mut self) {
        let operation = self.collect_operation(1);

        let [dest, _, _]  = operation.parameters;

        self.current_state[dest as usize] = 5;
    }

    fn output_instr(&mut self) {
        let mut operation = self.collect_operation(1);

        self.mode_switch(&mut operation, 1);
        let [output, _, _] = operation.parameters;

        println!("diagnostic code: {}", output);
    }

    fn collect_operation(&mut self, param_cnt: usize) -> Operation {
        let c_state = &self.current_state;
        let i_ptr = self.instr_ptr;

        if c_state.len() >= param_cnt + 1 && i_ptr >= c_state.len() - param_cnt - 1 {
            panic!("Not enough params left in memory for operation");
        }

        if param_cnt > 3 {
            panic!("only param cnt up to 3 is supported for param collection");
        }


        // collect op code
        let op_code = self.current_state[self.instr_ptr];
        self.instr_ptr += 1;

        let mut op = Operation { opcode: op_code, parameters: [0; 3] };

        // collect parameters
        for param_i in 0..param_cnt {
            op.parameters[param_i] = self.current_state[self.instr_ptr];
            self.instr_ptr += 1;
        }

        op
    }

    fn extract_op(mut opcode: i32) -> i32 {
        let mut op = 0;

        for digit in 0..2 {
            op += (opcode % 10) * (10 as i32).pow(digit);
            opcode /= 10;
        }

        op
    }

    fn mode_switch(&self, operation: &mut Operation, to_switch: u32) {
        let op_code = operation.opcode;
        let mut params: [i32; 3] = operation.parameters;

        for param_i in 0..to_switch {
            let switched_param = match IntCode::get_mode(op_code, param_i) {
                0 => self.current_state[params[param_i as usize] as usize],
                1 => continue, 
                _ => panic!("invalid mode encountered from code {} at {}", op_code, self.instr_ptr)
            };

            params[param_i as usize] = switched_param;
        }

        operation.parameters = params;
    }

    fn get_mode(opcode: i32, param_num: u32) -> i32 {
        // remove digits that identify op type and preceding params
        let stripped = opcode / (100 * (10 as i32).pow(param_num));

        // return last digit
        stripped % 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_op() {
        assert_eq!(IntCode::extract_op(111102), 2);

        assert_eq!(IntCode::extract_op(10), 10);

        assert_eq!(IntCode::extract_op(100059), 59);
    }

    #[test]
    fn get_mode() {
        let f = |opcode, param_num| { IntCode::get_mode(opcode, param_num) };

        assert_eq!(f(11102, 1), 1);
        assert_eq!(f(0, 3), 0);
        assert_eq!(f(90000, 2), 9);
        assert_eq!(f(300, 0), 3);
    }

    #[test]
    fn collect_params() {
        let memory = vec!(0, 1, 2, 3, 4, 5);

        let mut machine = IntCode {
            memory: memory.to_vec(),
            current_state: memory.to_vec(),
            instr_ptr: 0
        };

        let expected = [1, 2, 3];

        assert_eq!(expected, machine.collect_operation(3).parameters);
    }

    #[test]
    fn do_add() {
        let memory = vec!(1, 1, 1, 4, 4);

        let mut machine = IntCode {
            memory: Vec::new(),
            current_state: memory.to_vec(),
            instr_ptr: 0
        };

        machine.math_instr(|n1, n2| n1 + n2);

        assert_eq!(machine.current_state[4], 2);
        assert_eq!(machine.instr_ptr, 4)
    }

    #[test]
    fn mode_switch() {
        let memory = vec!(1001, 4, 78, 4, 50);

        let machine = IntCode {
            memory: Vec::new(),
            current_state: memory.to_vec(),
            instr_ptr: 0
        };

        let mut op = Operation {
            opcode: memory[0],
            parameters: [4, 78, 4]
        };

        machine.mode_switch(&mut op, 2);

        assert_eq!([50, 78, 4], op.parameters);
    }
}