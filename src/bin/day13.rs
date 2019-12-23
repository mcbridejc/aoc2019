use structopt::StructOpt;
use aoc2019::StandardOptions;
use aoc2019::intcode::{Executor, read_program_from_file};
use aoc2019::grid::{Grid, Direction, Location, xy};
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

fn part1(program: &Vec<i64>) -> i32 {
    let mut display = Grid::empty_with_default(Some(Tile::Empty));
    let mut m = Executor::new(program.clone());

    loop {
        let output = m.run_to_output_ntimes(3);
        if output.len() < 3 {
            break;
        }
        let x = output[0] as i32;
        let y = output[1] as i32;
        let tile = match output[2] {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("bad tile output")
        };
        display.set(&xy(x, y), Some(tile));
    }

    let mut block_count = 0;
    for (_, tile) in display.iter() {
        if tile.unwrap() == Tile::Block {
            block_count += 1;
        }
    }

    block_count
}

fn part2(program: &Vec<i64>) -> i64 {
    let mut program = program.clone();
    program[0] = 2; // put in a quarter
    let mut display = Grid::empty_with_default(Some(Tile::Empty));
    let mut m = Executor::new(program.clone());

    let mut ball_position = xy(0, 0);
    let mut paddle_position = xy(0, 0);
    let mut joystick: i64 = 0;
    let mut score: i64 = 0;
    let mut halted = false;
    loop {
        m.set_input(vec![joystick]);
        halted = !m.run_to_input();

        while(m.output.len() > 0) {
            let x = m.output.remove(0) as i32;
            let y = m.output.remove(0) as i32;
            let val = m.output.remove(0);
            
            if x == -1 && y == 0 {
                score = val;
            } else {
                let tile = match val {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => {paddle_position = xy(x, y); Tile::Paddle},
                    4 => {ball_position = xy(x, y); Tile::Ball},
                    _ => panic!("bad tile output")
                };
                display.set(&xy(x, y), Some(tile));
            }
        }
        
        // Yeah, I *could* avoid this counting iteration every time...but I bet it doesn't matter
        let mut block_count = 0;
        for (_, tile) in display.iter() {
            if tile.unwrap() == Tile::Block {
                block_count += 1;
            }
        }
        println!("Block count: {}", block_count);
        if block_count == 0 || halted {
            break;
        }

        if ball_position.x < paddle_position.x {
            joystick = -1;
        } else if ball_position.x > paddle_position.x {
            joystick = 1;
        } else {
            joystick = 0;
        }
    }

    
    return score;
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();
    
    let program = read_program_from_file(opt.input).unwrap();
    
    if opt.part1 {
        let count = part1(&program);
        println!("Number of blocks: {}", count);
    } else {
        let score = part2(&program);
        // > 12901
        println!("Final score: {}", score);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_ex1() {
    }
}