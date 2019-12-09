use std::cmp::max;

use structopt::StructOpt;
use aoc2019::intcode::{Executor, execute_program, read_program_from_file};
use aoc2019::StandardOptions;

const NUM_AMP: usize = 5;

fn permutations(values: &Vec<i32>) -> Vec<Vec<i32>> {
    if values.len() == 1 {
        return vec![vec![values[0]]];
    }

    let mut r: Vec<Vec<i32>> = vec![];
    for i in 0..values.len() {
        let v = vec![values[i]];
        let mut remaining_values = vec![];
        remaining_values.extend(&values[0..i]);
        remaining_values.extend(&values[i+1..]);

        for p in &permutations(&remaining_values) {
            let mut sample: Vec<i32> = v.clone();
            
            sample.extend(p);
            r.push(sample);
        }
    }
    r
}

fn part1(program: &Vec<i32>) -> i32 {
    let mut max_out = 0;
    for phase in permutations(&vec![0, 1, 2, 3, 4]) {
        let mut input = vec![0, 0];
        for i in 0..NUM_AMP {
            input[0] = phase[i];
            let (_mem, output) = execute_program(&program, &input);
            input[1] = output[0];
        }
        max_out = max(max_out, input[1]);
    }
    return max_out;
}

fn part2(program: &Vec<i32>) -> i32 {
    let mut max_out = 0;
    for phase in permutations(&vec![5, 6, 7, 8, 9]) {
        let mut boxen = vec![];
        for _ in 0..NUM_AMP {
            boxen.push(Executor::new(program.clone()));
        }

        // First round, we input phase
        let mut input = vec![0, 0];
        let mut amp_output = 0;
        for i in 0..NUM_AMP {
            input[0] = phase[i];
            boxen[i].set_input(input.clone());
            let output = boxen[i].run_to_output().unwrap(); 
            input[1] = output;
        }

        println!("Finished first cycle");

        let mut halted = false;
        let mut input = vec![input[1]];
        while !halted {
            for i in 0..NUM_AMP {
                boxen[i].set_input(input.clone());
                let output = boxen[i].run_to_output();
                if output.is_none() {
                    // Machine must have halted
                    halted = true;
                    break;
                } 
                input[0] = output.unwrap();
                if i == NUM_AMP-1 {
                    amp_output = input[0];
                }
            }
        }
        max_out = max(amp_output, max_out);
    }
    max_out
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

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
    use aoc2019::intcode::read_program_from_string;
    use crate::*;

    #[test]
    fn test_permutations() {
        let p = permutations(&vec![0, 1]);
        assert_eq!(p.len(), 2);
        assert!(p.contains(&vec![0, 1]));
        assert!(p.contains(&vec![1, 0]));

        let p = permutations(&vec![0, 1, 2]);
        assert_eq!(p.len(), 6);
        assert!(p.contains(&vec![0, 1, 2]));
        assert!(p.contains(&vec![1, 0, 2]));
        // Not doing full check
    }
    #[test]
    fn test_part1_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = read_program_from_string(program.to_string()).unwrap();
        let result = part1(&program);
        assert_eq!(result, 43210);
    }

    #[test]
    fn test_part2_1() {
        let program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                      27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let program = read_program_from_string(program.to_string()).unwrap();
        let result = part2(&program);
        assert_eq!(result, 139629729);
    }
    
}