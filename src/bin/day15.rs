use std::collections::HashMap;

use structopt::StructOpt;
use aoc2019::StandardOptions;
use aoc2019::intcode::{Executor, read_program_from_file};
use aoc2019::grid::{Grid, Direction, Location, xy};
use anyhow::Result;

fn try_move(m: &Executor, dir: Direction) -> Option<(Executor, bool)> {
    use Direction::*;

    let mut m = m.clone();
    let input: i64 = match dir {
        North => 1,
        South => 2,
        West => 3,
        East => 4
    };
    m.set_input(vec![input]);
    let output = m.run_to_output().unwrap();
    if output == 0 {
        None
    } else if output == 1 {
        Some((m, false))
    } else {
        Some((m, true)) // Found our target
    }
}


struct Room {
    loc: Location,
    m: Executor,
}

fn part1(program: &Vec<i64>) -> (i32, Executor) {
    let mut steps = 0;
    // Store a machine state for each room we find, i.e. each location is represented by 
    // the state of the Intcode machine when the "robot" is in that room
    // Visit each new adjacent room, expanding outward until we reach the target at which
    // point we know we took the shorted path. 
    let mut rooms: Vec<Room> = vec![Room{loc: xy(0,0), m: Executor::new(program.clone())}];
    let mut map: HashMap<Location, bool> = HashMap::new();
    loop {
        let mut next_rooms: Vec<Room> = vec![];
        steps += 1;
        for r in &rooms {
            for dir in Direction::iter() {
                let newloc = r.loc.go_one(*dir);
                if map.contains_key(&newloc) {
                    continue;
                }
                match(try_move(&r.m, *dir)) {
                    Some((m, target_reached)) => {
                        if target_reached {
                            return (steps, m);
                        }
                        next_rooms.push(Room{loc: newloc, m});
                        map.insert(newloc, true);
                    },
                    None => (),
                }
            }
        }
        rooms = next_rooms;
        next_rooms = vec![];
    }
}

fn part2(m: Executor) -> i32 {
    // Do the exact same thing as part 1, except start from the oxygen room we 
    // found in part 1, and run until we reach the farthest room
    let mut steps = 0;
    let mut rooms: Vec<Room> = vec![Room{loc: xy(0,0), m: m}];
    let mut map: HashMap<Location, bool> = HashMap::new();
    loop {
        let mut next_rooms: Vec<Room> = vec![];
        steps += 1;
        for r in &rooms {
            for dir in Direction::iter() {
                let newloc = r.loc.go_one(*dir);
                if map.contains_key(&newloc) {
                    continue;
                }
                match(try_move(&r.m, *dir)) {
                    Some((m, target_reached)) => {
                        next_rooms.push(Room{loc: newloc, m});
                        map.insert(newloc, true);
                    },
                    None => (),
                }
            }
        }
        rooms = next_rooms;
        next_rooms = vec![];
        if rooms.len() == 0 {
            break;
        }
    }
    // minus one because on the last iteration we banged into all walls or already visited rooms
    steps - 1
}


fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();
    
    let program = read_program_from_file(opt.input).unwrap();
    
    if opt.part1 {
        let (count, _) = part1(&program);
        println!("Distance: {}", count);
    } else {
        let (_, oxygen_room_machine) = part1(&program);
        let count = part2(oxygen_room_machine);
        // < 329
        println!("distance: {}", count);
    }
}