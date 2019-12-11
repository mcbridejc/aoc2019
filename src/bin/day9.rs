use structopt::StructOpt;
use aoc2019::StandardOptions;

use aoc2019::intcode::{read_program_from_file, Executor};


fn part1(program: &Vec<i64>) -> i64 {
    let input: Vec<i64> = vec![1];
    let mut m = Executor::new(program.clone());
    m.set_input(input);
    m.run();
    return m.output[0];
}

fn part2(program: &Vec<i64>) -> i64 {
    let input: Vec<i64> = vec![2];
    let mut m = Executor::new(program.clone());
    m.set_input(input);
    m.run();
    return m.output[0];
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    // Width and height are given in problem
    let width = 25;
    let height = 6; 

    let program = read_program_from_file(opt.input).unwrap();

    if opt.part1 {
        let result = part1(&program);
        println!("Answer: {}", result);
    } else {
        let result = part2(&program);
        println!("Answer: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use aoc2019::intcode::read_program_from_string;

    #[test]
    fn test_part1_ex1() {
        let program = read_program_from_string("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string()).unwrap();
        let mut m = Executor::new(program.clone());
        m.run();
        assert_eq!(program, m.output);
    }
}