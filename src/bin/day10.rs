use std::cmp::{min, max};
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

use num::integer::Integer;

use structopt::StructOpt;
use aoc2019::StandardOptions;
use aoc2019::grid::{Location, xy};
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    connections: i32,
}

#[derive(Clone, Debug)]
pub struct Map {
    map: HashMap<Location, Asteroid>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn get(&self, loc: &Location) -> Option<&Asteroid> {
        if !self.map.contains_key(loc) {
            return None;
        }
        self.map.get(loc)
    }

    pub fn best(&self) -> (Location, i32) {
        let mut best_loc: Location = xy(0,0);
        let mut best_count = 0;
        for loc in self.map.keys() {
            let asteroid = self.map.get(loc).unwrap();
            if asteroid.connections > best_count {
                best_count = asteroid.connections;
                best_loc = *loc;
            }
        }
        (best_loc, best_count)
    }
}

fn read_asteroid_map(filepath: &str) -> Result<Map> {
    let content = fs::read_to_string(filepath)?;
    read_asteroid_map_str(&content)
}

fn read_asteroid_map_str(content: &str) -> Result<Map> {
    let mut map: HashMap<Location, Asteroid> = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    let mut width = 0;
    for l in content.lines() {
        for c in l.chars() {
            if c == '#' {
                map.insert(xy(x, y), Asteroid{connections: 0});
            }
            x += 1;
            if x > width {
                width = x;
            }
        }
        x = 0;
        y += 1;
    }
    Ok(Map{map, width, height: y})
}

fn count_map(map: &Map) -> Map {
    let mut map = map.clone();

    let coords: Vec<Location> = map.map.keys().map(|x| {*x}).collect();
    
    let connect = |a, b| {
        map.map.get_mut(&a).unwrap().connections += 1;
        map.map.get_mut(&b).unwrap().connections += 1;
    };

    // Iterate through each asteroid, checking it against all others
    // We only check each against asteroids that come later in the list, 
    // because it will already have been compared to all of the ealier ones
    for i in 0..coords.len() {
        let mut a = coords[i];
        for j in i+1..coords.len() {
            //let mut b = map.map.get_mut(coords[j])?;
            let b = coords[j];
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let gcd = dx.gcd(&dy);
            let sx = dx / gcd;
            let sy = dy / gcd;
            
            let mut p = b;
            loop {
                p.x -= sx;
                p.y -= sy;
                if p == a {
                    // They can see each other
                    map.map.get_mut(&a).unwrap().connections += 1;
                    map.map.get_mut(&b).unwrap().connections += 1;
                }
                if map.get(&p).is_some() {
                    // Hit an obstructing asteroid
                    break;
                }
            }
        }
    }
    map
}

fn map_str(map: &Map) -> String {
    let mut s = "".to_string();
    for y in 0..map.height {
        for x in 0..map.width {
            match map.get(&xy(x, y)) {
                Some(asteroid) => s.push_str(&format!("{:^5}", &asteroid.connections)),
                None => s.push_str("  .  "),
            }
        }
        s.push_str("\n");
    }
    s
}

#[derive(Debug)]
struct Item {
    loc: Location,
    angle: f64,
    distance: i32,
}

fn part1(map: &Map) -> (Location, i32) {
    let map = count_map(&map);
    println!("{}", map_str(&map));
    map.best()
}

fn part2(map: &Map) -> (Location) {
    let (sensor, _) = part1(&map);

    // Collect a list of all other asteroids (excluding sensor), and annotate
    // with angle and distance while we're at it
    let mut others: Vec<Item> = map.map.keys().filter_map( |loc| {
        if *loc == sensor {
            None
        } else {
            let dx = loc.x - sensor.x;
            let dy = sensor.y - loc.y; // Inverted so up is 0deg
            let mut angle = (dx as f64).atan2(dy as f64);
            if angle < 0. {
                angle += 2. * std::f64::consts::PI;
            }
            let distance = dx.abs() + dy.abs();
            Some(Item{loc: *loc, angle, distance})
        }
    }).collect();

    // Sort the list by angle, then group into sub lists by the angle. 
    // Those sub lists are sorted by distance, so that we can pop off the closest
    // first.
    others.sort_by(|i, j| i.angle.partial_cmp(&j.angle).unwrap());
    let mut sorted_list: Vec<Vec<&Item>> = Vec::new();
    for (key, group) in &others.iter().group_by(|i| i.angle) {
        let mut list: Vec<&Item> = group.collect();
        list.sort_by(|i, j| j.distance.cmp(&i.distance));
        sorted_list.push(list);
    }
    
    let mut list_idx = 0;
    let mut last_loc = xy(0, 0);
    for _ in 0..200 {
        // Skip any empty angle slots
        while sorted_list[list_idx].len() == 0 {
            (list_idx + 1) % sorted_list.len();
        }
        
        last_loc = sorted_list[list_idx].pop().unwrap().loc;
        list_idx = (list_idx + 1) % sorted_list.len();
    }
    last_loc
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();
    
    let map = read_asteroid_map(&opt.input).unwrap();
    
    if opt.part1 {
        let (loc, score) = part1(&map);
        println!("Best location: {:?} with {} asteroids", loc, score);
    } else {
        let loc = part2(&map);
        println!("200th asteroid: {:?}", loc);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_ex1() {
        let map = count_map(&read_asteroid_map("input/day10/ex1.txt").unwrap());
        let (best_loc, count) = map.best();
        println!("{}",map_str(&map));
        assert_eq!(best_loc, xy(5, 8));
        assert_eq!(count, 33);
    }
    #[test]
    fn test_part2_bigex() {
        let map = read_asteroid_map("input/day10/bigex.txt").unwrap();
        let loc = part2(&map);
        assert_eq!(loc, xy(8, 2));
    }
}