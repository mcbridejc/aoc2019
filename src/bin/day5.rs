use structopt::StructOpt;
use aoc2019::intcode::{execute_program, read_program_from_file, read_program_from_string};
use aoc2019::StandardOptions;

fn part1(program: Vec<i64>) -> i64 {
    let input: Vec<i64> = vec![1];
    let (_mem, output) = execute_program(&program, &input);
    println!("Program output: {:?}",  output);
    output[output.len()-1]
}

fn part2(program: Vec<i64>) -> i64 {
    let input: Vec<i64> = vec![5];
    let (_mem, output) = execute_program(&program, &input);
    println!("Program output: {:?}",  output);
    output[output.len()-1]
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    let program = read_program_from_file(opt.input).unwrap();

    if opt.part1 {
        part1(program);
    } else {
        part2(program);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let expected_result = 15259545;
        let program = read_program_from_string(include_str!("../../input/day5/input.txt").to_string());
        let result = part1(program.unwrap());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let expected_result = 7616021;
        let program = read_program_from_string(include_str!("../../input/day5/input.txt").to_string());
        let result = part2(program.unwrap());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_case1() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let program = read_program_from_string(program.to_string()).unwrap();

        println!("--- Testing input 7");
        let input = vec![7];
        let (_mem, output) = execute_program(&program, &input);
        assert_eq!(output[0], 999);
        println!("--- Testing input 8");
        let input = vec![8];
        let (_mem, output) = execute_program(&program, &input);
        assert_eq!(output[0], 1000);
        println!("--- Testing input 9");
        let input = vec![9];
        let (_mem, output) = execute_program(&program, &input);
        assert_eq!(output[0], 1001);
    }
}