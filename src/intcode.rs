use std::fs;

use anyhow::Result;
use log::*;


#[derive(Debug)]
enum Instruction {
    Add (i32, i32, i32),
    Mult (i32, i32, i32),
    Input (i32),
    Output (i32),
    JmpTrue (i32, i32),
    JmpFalse (i32, i32),
    CmpLt (i32, i32, i32),
    CmpEq (i32, i32, i32),
    Stop,
}

impl Instruction {
    // There doesn't seem to be a built in way to find the lenth of a tuple variant. Rust is wierd. 
    pub fn len(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mult(_, _, _) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::JmpTrue(_, _) => 3,
            Instruction::JmpFalse(_, _) => 3,
            Instruction::CmpLt(_, _, _) => 4,
            Instruction::CmpEq(_, _, _) => 4,
            Instruction::Stop => 1,
        }
    }

    pub fn run(&self, m: &mut Executor) -> u32 {
        let mut new_pc = std::u32::MAX;
        match self {
            Instruction::Add(a, b, out) => m.mem[*out as usize] = a + b,
            Instruction::Mult(a, b, out) => m.mem[*out as usize] = a * b,
            Instruction::Input(out) => m.mem[*out as usize] = m.read_input(),
            Instruction::Output(a) => m.write_output(*a),
            Instruction::JmpTrue(a, addr) => {
                if *a != 0 {
                    new_pc = *addr as u32;
                }
            },
            Instruction::JmpFalse(a, addr) => {
                if *a == 0 {
                    new_pc = *addr as u32;
                }
            },
            Instruction::CmpLt(a, b, out) => {
                let y = if *a < *b {
                    1
                } else {
                    0
                };
                m.mem[*out as usize] = y;
            },
            Instruction::CmpEq(a, b, out) => {
                let y = if *a == *b {
                    1
                } else {
                    0
                };
                m.mem[*out as usize] = y;
            }
            Instruction::Stop => m.halted = true,
        }
        if new_pc < std::u32::MAX {
            new_pc
        } else {
            m.pc + self.len() as u32
        }
    }
}

fn imm_mask_from_cmd(cmd: u32) -> u32 {
    let mut c = cmd / 100; // Drop the bottom two decimal digits, these are opcode
    let mut mask = 0u32;
    let mut b = 0;
    while c > 0 {
        match c % 10 {
            0 => (),
            1 => mask |= 1<<b,
            _ => panic!("Unrecognized argument code: {}", c),
        }
        c = c / 10;
        b += 1;
    }
    mask
}

struct Executor {
    pc: u32,
    mem: Vec<i32>,
    input: Vec<i32>,
    inptr: usize,
    output: Vec<i32>,
    halted: bool
}

impl Executor {
    pub fn new(program: Vec<i32>) -> Executor {
        Executor{pc: 0, mem: program, output: Vec::<i32>::new(), input: Vec::<i32>::new(), inptr: 0, halted: false}
    }

    pub fn set_input(&mut self, input: Vec<i32>) {
        self.input = input;
        self.inptr = 0;
    }

    pub fn read_input(&mut self) -> i32 {
        self.inptr += 1;
        self.input[self.inptr-1]
    }

    pub fn write_output(&mut self, x: i32) {
        self.output.push(x);
        println!("Out: {}", x);
    }

    pub fn load(&self) -> Instruction {
        let mem = &self.mem;
        let pc = self.pc as usize;
        let cmd = mem[pc] as u32;
        let opcode = cmd % 100;
        let imm_mask = imm_mask_from_cmd(cmd);

        println!("Immediate mask: {}", imm_mask);
        
        let mut pos = 0u32;

        let mut read_argument = |output: bool| -> i32 {
            let arg = mem[pc + pos  as usize + 1];
            if output { // Outputs are always returned as address
                pos += 1;
                return arg;
            }
            let result = if imm_mask & (1<<pos) != 0 {
                arg
            } else {
                if arg < 0 || arg as usize >= mem.len() {
                    self.dump("".to_string());
                    panic!("Out of range memory access. PC: {}, ADDR: {}", pc, arg);
                }
                mem[arg as usize]
            };
            pos += 1;
            result
        };

        let instr = match opcode {
            1 => Instruction::Add(read_argument(false), read_argument(false), read_argument(true)),
            2 => Instruction::Mult(read_argument(false), read_argument(false), read_argument(true)),
            3 => Instruction::Input(read_argument(true)),
            4 => Instruction::Output(read_argument(false)),
            5 => Instruction::JmpTrue(read_argument(false), read_argument(false)),
            6 => Instruction::JmpFalse(read_argument(false), read_argument(false)),
            7 => Instruction::CmpLt(read_argument(false), read_argument(false), read_argument(true)),
            8 => Instruction::CmpEq(read_argument(false), read_argument(false), read_argument(true)),
            99 => Instruction::Stop{},
            _ => {
              self.dump(format!("Unrecognized instruction: {} @ PC={}", opcode, pc));
              panic!("Aborting");  
            },
        };
        println!("{}: {:?}", pc, instr);
        instr
    }

    pub fn execute(&mut self, i: Instruction) {
        self.pc = i.run(self);
    }

    pub fn dump(&self, msg: String) {
        debug!("Performing memory dump of {} words", self.mem.len());
        let mut msg: String = msg;
        msg += "\nMemory Dump: \n";
        let mut addr = 0;
        while addr < self.mem.len() {
            if addr % 10 == 0 {
                msg.push_str(&format!("\n{}: ", addr)); 
            }
            msg.push_str(&format!(" {}", self.mem[addr]));
            addr += 1;
            debug!("Printing addr {}, {}", addr, msg.len());
        }
        println!("{}", msg);
    }
}

pub fn read_program_from_string(s: String) -> Result<Vec<i32>> {
    let ints: Vec<i32> = s.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    Ok(ints)
}

pub fn read_program_from_file(file: String) -> Result<Vec<i32>> {
    let content = fs::read_to_string(file)?;
    read_program_from_string(content)
}

pub fn execute_program(program: &Vec<i32>, input: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut exec = Executor::new(program.clone());
    exec.set_input(input.clone());
    while !exec.halted {
        exec.execute(exec.load());
    }
    
    (exec.mem, exec.output)
}

#[cfg(test)]
mod tests {
    use crate::intcode::*;
    #[test]
    fn test_instr_length() {
        let inst = Instruction::Input(12);
        assert_eq!(inst.len(), 2);
    }

    
}