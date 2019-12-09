use std::fs;

use structopt::StructOpt;

use anyhow::Result;
use aoc2019::StandardOptions;
use aoc2019::intcode::execute_program;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(flatten)]
    shared: StandardOptions,

    #[structopt(long="demo", help="Just an example of extending standard options")]
    demo: bool,
}

fn parse_comma_separated(s: String) -> Result<Vec<i32>> {
    let ints: Vec<i32> = s.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    //println!("ints: {:?}", ints);
    Ok(ints)
}

fn read_comma_separated(file: String) -> Result<Vec<i32>> {
    let content = fs::read_to_string(file)?;
    
    parse_comma_separated(content)
}


fn main() {
    let opt = Options::from_args();

    println!("Reading from {}", opt.shared.input);
    let mut program = read_comma_separated(opt.shared.input).unwrap();
    
    if opt.shared.part1 {
        // "before running the program, replace position 1 with the value 12 and replace position 2 with the value 2"
        program[1] = 12;
        program[2] = 2;
        let (result, _) = execute_program(&program, &Vec::<i32>::new());
        println!("Result: {}", result[0]);
        
    } else {
        let target_result = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                program[1] = noun;
                program[2] = verb;
                let (result, _) = execute_program(&program, &Vec::<i32>::new());
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

    fn test_vector(program: Vec<i32>, expected: Vec<i32>) {
        let (output, _) = execute_program(&program, &Vec::<i32>::new());
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