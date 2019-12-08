use std::fs;

use structopt::StructOpt;

use anyhow::Result;
use aoc2019::StandardOptions;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(flatten)]
    shared: StandardOptions,

    #[structopt(long="demo", help="Just an example of extending standard options")]
    demo: bool,
}

fn parse_comma_separated(s: String) -> Result<Vec<u32>> {
    let ints: Vec<u32> = s.trim().split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    //println!("ints: {:?}", ints);
    Ok(ints)
}

fn read_comma_separated(file: String) -> Result<Vec<u32>> {
    let content = fs::read_to_string(file)?;
    
    parse_comma_separated(content)
}

enum Instruction {
    Add { a: u32, b: u32, out: u32 },
    Mult{ a: u32, b: u32, out: u32 },
    Stop
}

struct Executor {
    pc: u32,
    mem: Vec<u32>,
    halted: bool
}

impl Executor {
    pub fn load(&self) -> Instruction {
        
        let mem = &self.mem;
        let pc = self.pc as usize;
        let opcode = mem[pc];

        match opcode {
            1 => Instruction::Add{a: mem[pc + 1], b: mem[pc+2], out: mem[pc+3]},
            2 => Instruction::Mult{a: mem[pc + 1], b: mem[pc+2], out: mem[pc+3]},
            99 => Instruction::Stop{},
            _ => panic!("Unrecognized instruction: {}", opcode),
        }
    }

    pub fn execute(&mut self, i: Instruction) {
        match i {
            Instruction::Add{a, b, out} => self.mem[out as usize] = self.mem[a as usize] + self.mem[b as usize],
            Instruction::Mult{a, b, out} => self.mem[out as usize] = self.mem[a as usize] * self.mem[b as usize],
            Instruction::Stop => { self.halted = true; return }
            _ => panic!("Unimplemented instruction"),
        }
        self.pc += 4;
    }
}

fn execute_program(input: &Vec<u32>) -> Vec<u32> {
    let mut exec = Executor { pc: 0, mem: input.clone(), halted: false };
    while !exec.halted {
        exec.execute(exec.load());
    }
    
    exec.mem
}

fn main() {
    let opt = Options::from_args();

    println!("Reading from {}", opt.shared.input);
    let mut program = read_comma_separated(opt.shared.input).unwrap();
    
    if opt.shared.part1 {
        // "before running the program, replace position 1 with the value 12 and replace position 2 with the value 2"
        program[1] = 12;
        program[2] = 2;
        let result = execute_program(&program);
        println!("Result: {}", result[0]);
        
    } else {
        let target_result = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                program[1] = noun;
                program[2] = verb;
                let result = execute_program(&program);
                if result[0] == target_result {
                    println!("Found input noun={}, verb = {}", noun, verb);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn test_vector(input: Vec<u32>, expected: Vec<u32>) {
        let output = execute_program(&input);
        assert_eq!(output, expected);
    }
    #[test]
    fn examples() {
        test_vector(vec![1,0,0,0,99], vec![2,0,0,0,99]);
        test_vector(vec![2,3,0,3,99], vec![2,3,0,6,99]);
        test_vector(vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]);
        test_vector(vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]);
    }
}