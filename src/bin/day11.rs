use structopt::StructOpt;
use aoc2019::StandardOptions;
use aoc2019::intcode::{Executor, read_program_from_file};
use aoc2019::grid::{Grid, Direction, Location, xy};
use anyhow::Result;

#[derive(Clone)]
struct GridCell {
    white: bool,
    painted: bool,
}

fn turn(dir: Direction, turn_cmd: i64) -> Direction {
    use Direction::*;
    // If turn_cmd == 0 we turn left, if == 1 we turn right
    match dir {
        North => if turn_cmd == 0 {West} else {East},
        East => if turn_cmd == 0 {North} else {South},
        South => if turn_cmd == 0 {East} else {West},
        West => if turn_cmd == 0 {South} else {North},
    }
}

fn run_the_painting_robot(map: &mut Grid<GridCell>, m: &mut Executor) {
    use Direction::*;
    let mut p = xy(0,0); // Start robot at 0,0
    let mut dir: Direction = North; // Start robot facing north
    loop {
        let cur_cell = map.get(&p).unwrap();
        let input: Vec<i64> = if cur_cell.white {
            vec![1]
        } else {
            vec![0]
        };
        m.set_input(input);
        let paint_cmd = match m.run_to_output() {
            Some(result) => result,
            None => break,
        };
        let turn_cmd = match m.run_to_output() {
            Some(result) => result,
            None => break,
        };
        map.set(&p, Some(GridCell{white: paint_cmd == 1, painted: true}));
        dir = turn(dir, turn_cmd);
        p = p.go_one(dir);
    }
}

fn part1(program: &Vec<i64>) -> i64 {


    let mut map = Grid::empty_with_default(Some(GridCell{white: false, painted: false}));
    let mut m = Executor::new(program.clone());
    
    run_the_painting_robot(&mut map, &mut m);

    let mut painted_count = 0;
    for (location, cell) in map.iter() {
        if cell.is_some() && cell.unwrap().painted {
            painted_count += 1;
        }
    }

    return painted_count;
    
}

fn part2(program: &Vec<i64>) {
    let mut map = Grid::empty_with_default(Some(GridCell{white: false, painted: false}));
    let mut m = Executor::new(program.clone());
    map.set(&xy(0, 0), Some(GridCell{white: true, painted: false}));

    run_the_painting_robot(&mut map, &mut m);

    for y in map.top..map.top+map.height {
        let mut content = false;
        for x in map.left..map.left+map.width {
            if map.get(&xy(x, y)).unwrap().white {
                print!("o");
                content = true;
            } else {
                print!(" ");
            }
        }
        if content {
            print!("\n");
        }
    }
}


fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();
    
    let program = read_program_from_file(opt.input).unwrap();
    
    if opt.part1 {
        let count = part1(&program);
        println!("Number of painted cells: {}", count);
    } else {
        part2(&program);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_ex1() {
    }
}