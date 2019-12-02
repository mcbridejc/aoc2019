use structopt::StructOpt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

//use log::*;
//use crate::errors::Result;
use aoc2019::errors::*;

#[derive(Debug, StructOpt)]
struct Options {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Run part
    #[structopt(short="o", long="one", help="Run part 1", conflicts_with("part2"), required_unless("part2"))]
    part1: bool,

    /// Run part2
    #[structopt(short="t", long="two", help="Run part 2", conflicts_with("part1"), required_unless("part1"))]
    part2: bool,

    /// Input data file
    #[structopt(short, long)]
    input: String,
}

fn mass_to_fuel(m: u32) -> u32 {
    let f = m / 3;
    if f < 2 {
        0
    } else {
        f - 2
    }
}

fn recursive_mass_to_fuel(m: u32) -> u32 {
    let mut f = mass_to_fuel(m);
    let mut extra = mass_to_fuel(f);
    f += extra;
    while extra > 0 {
        extra = mass_to_fuel(extra);
        f += extra;
    }
    f
}

fn read_numbers(file: String) -> Result<Vec<i32>> {
    let f = File::open(file).chain_err(|| "couldn't open input file")?;
    let reader = BufReader::new(f);
    let mut v = Vec::<i32>::new();
    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        v.push(num);
    }
    Ok(v)
}

fn main() {
    let opt = Options::from_args();

    println!("Reading from {}", opt.input);
    let list = read_numbers(opt.input).unwrap();
    
    if opt.part1 {
        println!("Running part 1");    
        let mut total_fuel = 0;
        for x in list {
            total_fuel += mass_to_fuel(x as u32);
        }
        println!("Total fuel: {}", total_fuel);

    } else {
        println!("Running part 2");
        let mut total_fuel = 0;
        for x in list {
            total_fuel += recursive_mass_to_fuel(x as u32);
        }
        println!("Total fuel: {}", total_fuel);
    }
}