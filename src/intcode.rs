use std::fs;
use std::mem::discriminant;

use anyhow::Result;
use log::*;

type Program = Vec<i64>;

#[derive(Debug)]
pub enum Instruction {
    Add (i64, i64, i64),
    Mult (i64, i64, i64),
    Input (i64),
    Output (i64),
    JmpTrue (i64, i64),
    JmpFalse (i64, i64),
    CmpLt (i64, i64, i64),
    CmpEq (i64, i64, i64),
    SetBase (i64),
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
            Instruction::SetBase(_) => 2,
            Instruction::Stop => 1,
        }
    }

    pub fn run(&self, m: &mut Executor) -> u32 {
        let mut new_pc = std::u32::MAX;
        use Instruction::*;
        match self {
            Add(a, b, out) => m.write_mem(*out as usize, a + b),
            Mult(a, b, out) => m.write_mem(*out as usize, a * b),
            Input(out) => {
                let input = m.read_input();
                m.write_mem(*out as usize, input);
            },
            Output(a) => m.write_output(*a),
            JmpTrue(a, addr) => {
                if *a != 0 {
                    new_pc = *addr as u32;
                }
            },
            JmpFalse(a, addr) => {
                if *a == 0 {
                    new_pc = *addr as u32;
                }
            },
            CmpLt(a, b, out) => {
                let y = if *a < *b {
                    1
                } else {
                    0
                };
                m.write_mem(*out as usize, y);
            },
            CmpEq(a, b, out) => {
                let y = if *a == *b {
                    1
                } else {
                    0
                };
                m.write_mem(*out as usize, y);
            },
            SetBase(a) => m.base_reg += *a,
            Stop => m.halted = true,
        }
        if new_pc < std::u32::MAX {
            new_pc
        } else {
            m.pc + self.len() as u32
        }
    }
}

enum ArgMode {
    Absolute = 0,
    Immediate = 1,
    Relative = 2,
}

fn arg_mode(cmd: u32, arg: u32) -> ArgMode {
    // The first 10^2 is for the opcode two digits
    let digit = cmd / 10u32.pow((2 + arg) as u32) % 10;
    match digit {
        0 => ArgMode::Absolute,
        1 => ArgMode::Immediate,
        2 => ArgMode::Relative,
        _ => panic!("Unallowed argument flag"),
    }
}

pub struct Executor {
    pc: u32,
    mem: Vec<i64>,
    base_reg: i64,
    input: Vec<i64>,
    inptr: usize,
    pub output: Vec<i64>,
    halted: bool
}

impl Executor {
    pub fn new(program: Vec<i64>) -> Executor {
        Executor{pc: 0, mem: program, output: Vec::<i64>::new(), input: Vec::<i64>::new(), inptr: 0, halted: false, base_reg: 0}
    }

    pub fn read_mem(&mut self, addr: usize) -> i64 {
        // Grow memory to accomodate if needed
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr]
    }

    pub fn write_mem(&mut self, addr: usize, value: i64) {
        // Grow memory to accomodate as needed
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = value;
    }

    pub fn set_input(&mut self, input: Vec<i64>) {
        self.input = input;
        self.inptr = 0;
    }

    pub fn read_input(&mut self) -> i64 {
        self.inptr += 1;
        println!("Reading input value {}", self.input[self.inptr-1]);
        self.input[self.inptr-1]
    }

    pub fn write_output(&mut self, x: i64) {
        self.output.push(x);
        println!("Out: {}", x);
    }

    pub fn load(&mut self) -> Instruction {
        let pc = self.pc as usize;
        let cmd = self.read_mem(pc) as u32;
        let opcode = cmd % 100;
        
        let mut pos = 0u32;

        let mut read_argument = |output: bool| -> i64 {
            let arg = self.read_mem(pc + pos  as usize + 1);
            let result = if output {
                match arg_mode(cmd, pos) {
                    ArgMode::Immediate => panic!("Cannot use immediate mode for output arg"),
                    ArgMode::Absolute => arg,
                    ArgMode::Relative => arg + self.base_reg,
                }
            } else {
                match arg_mode(cmd, pos) {
                    ArgMode::Immediate => arg,
                    ArgMode::Absolute => self.read_mem(arg as usize),
                    ArgMode::Relative => self.read_mem((arg + self.base_reg) as usize),
                }
            };
            pos += 1;
            result
        };

        use Instruction::*;
        let instr = match opcode {
            1 => Add(read_argument(false), read_argument(false), read_argument(true)),
            2 => Mult(read_argument(false), read_argument(false), read_argument(true)),
            3 => Input(read_argument(true)),
            4 => Output(read_argument(false)),
            5 => JmpTrue(read_argument(false), read_argument(false)),
            6 => JmpFalse(read_argument(false), read_argument(false)),
            7 => CmpLt(read_argument(false), read_argument(false), read_argument(true)),
            8 => CmpEq(read_argument(false), read_argument(false), read_argument(true)),
            9 => SetBase(read_argument(false)),
            99 => Stop{},
            _ => {
              self.dump(format!("Unrecognized instruction: {} @ PC={}", opcode, pc));
              panic!("Aborting");  
            },
        };
        println!("{}: {:?}", pc, instr);
        instr
    }

    pub fn execute(&mut self, i: &Instruction) {
        self.pc = i.run(self);
    }

    /// Run program until it halts
    pub fn run(&mut self) {
        loop {
            let instruction = self.load();
            self.execute(&instruction);
            if self.halted {
                break;
            }
        }
    }

    pub fn run_to_output(&mut self) -> Option<i64> {
        loop {
            let instruction = self.load();
            self.execute(&instruction);
            if discriminant(&instruction) == discriminant(&Instruction::Output(0)) {
                return Some(*self.output.last().unwrap());
            }
            if self.halted {
                return None;
            }
        }
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

pub fn read_program_from_string(s: String) -> Result<Vec<i64>> {
    let ints: Vec<i64> = s.trim().split(",").map(|s| s.parse::<i64>().unwrap()).collect();
    Ok(ints)
}

pub fn read_program_from_file(file: String) -> Result<Vec<i64>> {
    let content = fs::read_to_string(file)?;
    read_program_from_string(content)
}

pub fn execute_program(program: &Vec<i64>, input: &Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut exec = Executor::new(program.clone());
    exec.set_input(input.clone());
    while !exec.halted {
        let i = exec.load();
        exec.execute(&i);
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