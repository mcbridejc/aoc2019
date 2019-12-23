use std::collections::HashMap;
use std::fs;
use std::ops;

use num::integer::lcm;
use regex::Regex;
use structopt::StructOpt;


use aoc2019::StandardOptions;



#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

#[derive(Clone, Debug)]
struct Celestial {
    pos: Vec3,
    vel: Vec3,
}

fn read_initial(filename: &str) -> Vec<Celestial> {
    let content = fs::read_to_string(filename).unwrap();
    read_initial_str(&content)
}

fn read_initial_str(content: &str) -> Vec<Celestial> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let text_lines = content.split("\n");
    let mut celestials = Vec::<Celestial>::new();
    for line in text_lines {
        if line.len() == 0 {
            continue;
        }
        let caps = re.captures(line).unwrap();
        let x = caps[1].parse::<i64>().unwrap(); 
        let y = caps[2].parse::<i64>().unwrap();
        let z = caps[3].parse::<i64>().unwrap();
        celestials.push(Celestial{pos: Vec3{x, y, z}, vel: Vec3{x: 0, y: 0, z: 0}});
    }
    celestials
}


fn step(bodies: &mut Vec<Celestial>) {
    fn compare(a: i64, b: i64) -> (i64, i64) {
        if a > b {
            return (-1, 1);
        } else if a < b {
            return (1, -1);
        } else {
            return (0, 0);
        }
    }

    for i in 0..bodies.len() {
        for j in i+1..bodies.len() {
            let (di, dj) = compare(bodies[i].pos.x, bodies[j].pos.x);
            bodies[i].vel.x += di;
            bodies[j].vel.x += dj;
            let (di, dj) = compare(bodies[i].pos.y, bodies[j].pos.y);
            bodies[i].vel.y += di;
            bodies[j].vel.y += dj;
            let (di, dj) = compare(bodies[i].pos.z, bodies[j].pos.z);
            bodies[i].vel.z += di;
            bodies[j].vel.z += dj;
        }
    }

    for i in 0..bodies.len() {
        let vel = bodies[i].vel;
        bodies[i].pos += &vel;
    }
}

fn energy(bodies: &Vec<Celestial>) -> i64 {
    let mut energy = 0;
    for b in bodies {
        let potential = b.pos.x.abs() + b.pos.y.abs() + b.pos.z.abs();
        let kinetic = b.vel.x.abs() + b.vel.y.abs() + b.vel.z.abs();
        energy += potential * kinetic; // Because as we all know, total energy is the product of potential and kinetic energy :/
    }
    energy
}


fn part1(bodies: Vec<Celestial>, steps: usize) -> i64 {

    let mut bodies = bodies.clone();
    for _ in 0..steps {
        step(&mut bodies);
    }
    
    energy(&bodies)
}

// fn period(a: i64, b: i64) -> i64 {
//     // For each 1d oscillator (i.e. each axis for each pair of bodies) the bodies start with zero velocity
//     // and fall towards each other. They increase speed linearly until they cross at cycle N, at which
//     // point their velocity will be N. It will take N more cycles for them to reach 0 velocity at their 
//     // opposite displacement peaks, and then N more cycles to accelerate back through their crossing, 
//     // and finally another N cycles to reach zero velocity again, at which point they will both be back
//     // where they started. 
//     // So period is 4*N. 
//     //
//     // displacement on cycle n is n * (n+1) / 2 (the sum of all integers <= n)
    
//     // Special case: If we start on the same position

//     // Find the cycle on which the two pairs cross
//     let delta = (b - a).abs();
//     let mut N = 0;
//     loop {
//         N += 1;


//     }
// }

#[derive(Debug, Hash, PartialEq, Eq)]
struct OneAxisHash {
    pos: Vec<i64>,
    vel: Vec<i64>,  
}


fn part2(bodies: Vec<Celestial>) -> i64 {
    // Find the period of the universe

    // So I think the trick here is that the motion of each body, in each axis, is independent.
    // So we just need to find the period of each dimensions, and then find the 
    // least common multiple of the set of periods


    let mut bodies = bodies.clone();

    let mut x_hash: HashMap<OneAxisHash, i32> = HashMap::new();
    let mut y_hash: HashMap<OneAxisHash, i32> = HashMap::new();
    let mut z_hash: HashMap<OneAxisHash, i32> = HashMap::new();
    let mut x_period = -1;
    let mut y_period = -1;
    let mut z_period = -1;

    let n_bodies = bodies.len();
    let mut cycle = 0;
    while x_period < 0 || y_period < 0 || z_period < 0 {
        
        if x_period < 0 {
            let mut state = OneAxisHash{pos: Vec::with_capacity(n_bodies), vel: Vec::with_capacity(n_bodies)};
            for b in &bodies {
                state.pos.push(b.pos.x);
                state.vel.push(b.vel.x);
            }
            if x_hash.contains_key(&state) {
                x_period = cycle - x_hash.get(&state).unwrap();
            }
            x_hash.insert(state, cycle);
        }

        if y_period < 0 {
            let mut state = OneAxisHash{pos: Vec::with_capacity(n_bodies), vel: Vec::with_capacity(n_bodies)};
            for b in &bodies {
                state.pos.push(b.pos.y);
                state.vel.push(b.vel.y);
            }
            if y_hash.contains_key(&state) {
                y_period = cycle - y_hash.get(&state).unwrap();
            }
            y_hash.insert(state, cycle);
        }

        if z_period < 0 {
            let mut state = OneAxisHash{pos: Vec::with_capacity(n_bodies), vel: Vec::with_capacity(n_bodies)};
            for b in &bodies {
                state.pos.push(b.pos.z);
                state.vel.push(b.vel.z);
            }
            if z_hash.contains_key(&state) {
                z_period = cycle - z_hash.get(&state).unwrap();
            }
            z_hash.insert(state, cycle);
        }  

        step(&mut bodies);
        cycle += 1;
    }

    println!("x,y,z periods: {}, {}, {}", x_period, y_period, z_period);
    let period = lcm(lcm(x_period as i64, y_period as i64), z_period as i64);
    period
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();
    
    let initial = read_initial(&opt.input);
    
    if opt.part1 {
        const N: usize = 1000;
        let energy = part1(initial.clone(), N);
        println!("Energy: {}", energy);
    } else {
        let period = part2(initial.clone());
        println!("Total period: {}", period);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_part1_ex1() {
        let input = "<x=-1, y=0, z=2>\n\
                     <x=2, y=-10, z=-7>\n\
                     <x=4, y=-8, z=8>\n\
                     <x=3, y=5, z=-1>\n".to_string();
        let initial = read_initial_str(&input);

        let energy = part1(initial, 10);
        assert_eq!(energy, 179);
    }

    #[test]
    fn test_part2_ex1() {
        let input = "<x=-1, y=0, z=2>\n\
                     <x=2, y=-10, z=-7>\n\
                     <x=4, y=-8, z=8>\n\
                     <x=3, y=5, z=-1>\n".to_string();
        let initial = read_initial_str(&input);
        let period = part2(initial);
        assert_eq!(period, 2772);
    }

    #[test]
    fn test_part2_ex2() {
        let input = "<x=-8, y=-10, z=0>\n\
                    <x=5, y=5, z=10>\n\
                    <x=2, y=-7, z=3>\n\
                    <x=9, y=-8, z=-3>\n".to_string();
        let initial = read_initial_str(&input);
        let period = part2(initial);
        assert_eq!(period, 4686774924);
    }
}