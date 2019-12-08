
use std::str::FromStr;
use std::cmp::{min, max};

use structopt::StructOpt;

use aoc2019::StandardOptions;

use anyhow::{Error};

use aoc2019::io::{read_data_2d, read_data_2d_str};
use aoc2019::grid::{Grid, xy, Direction, Location};

#[derive(Debug)]
struct Step {
    dir: Direction,
    length: u32,
}

impl FromStr for Step {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dir = match &s[0..1] {
            "R" => Direction::East,
            "L" => Direction::West,
            "U" => Direction::North,
            "D" => Direction::South,
            _ => panic!("invalid direction character: {}" , s),
        };
        let length = s[1..].parse::<u32>();
        let length = match length {
            Ok(l) => l,
            Err(e) => panic!("Couldn't parse Step from '{}': {}", s, e),
        };

        Ok(Step{dir, length})
    }
}

#[derive(Debug)]
struct Segment {
    x0: i32,
    y0: i32,
    x1: i32, 
    y1: i32,
    d: i32, // Distance of the start of this segment from the origin of the wire
}

fn line_to_segments(steps: &Vec<Step>) -> Vec<Segment> {
    let mut p = xy(0, 0);
    let mut d = 0i32;
    let mut r = Vec::with_capacity(steps.len());
    for step in steps {
        let x0 = p.x;
        let y0 = p.y;
        p = p.go(step.dir, step.length as i32);
        r.push(Segment{x0, y0, x1: p.x, y1: p.y, d});
        d += step.length as i32;
    }
    r
}

/// Given two segments, find the point of overlap, if it exists
/// 
/// In cases of parallel segments, there may be many points of overlap. In this
/// case, we return the point closes to the (x0,y0) end of segment 1. For out
/// purposes, because we are scoring the sum of the distance along both wires,
/// this will always be the same distance as the earliest point: 
/// Case 1: If the two lines are in the same direction, closest go seg1.{x0, y0} 
/// will also be closest to seg2 origin
/// Case 2: If the two lines are in opposite directions, any point on the interval
/// of overlap will have the same distance score because moving in either direction
/// increments the score of one wire, while decrementing the score of the other
fn find_intersect(seg1: &Segment, seg2: &Segment) -> Option<Location> {
    let horizontal1 = seg1.y0 == seg1.y1;
    let horizontal2 = seg2.y0 == seg2.y1;
    let mut s1_n0: i32;
    let mut s1_n1: i32;
    let mut s2_n0: i32;
    let mut s2_n1: i32;

    if horizontal1 == horizontal2 {
        if horizontal1 {
            if seg1.y0 == seg2.y0 {
                s1_n0 = seg1.x0; 
                s1_n1 = seg1.x1;
                s2_n0 = seg2.x0;
                s2_n1 = seg2.x1;
            } else {
                return None;
            }
        } else {
            // vertical
            if seg1.x0 == seg2.x0 {
                s1_n0 = seg1.x0; 
                s1_n1 = seg1.x1;
                s2_n0 = seg2.x0;
                s2_n1 = seg2.x1;
            } else {
                return None;
            }
        }

        if s1_n0 > s1_n1 {
            let tmp = s1_n0;
            s1_n0 = s1_n1;
            s1_n1 = tmp;
        }

        let mut meet_distance = std::i32::MAX;
        if s2_n0 >= s1_n0 && s2_n0 <= s1_n1 {
            meet_distance = s2_n0 - s1_n0;
        }
        if s2_n1 >= s1_n0 && s2_n1 <= s1_n1 {
            meet_distance = min(meet_distance, s2_n1 - s1_n0);
        }
        if s2_n0 > s2_n1 {
            let tmp = s2_n0;
            s2_n0 = s2_n1;
            s2_n1 = tmp;
        }
        if s2_n0 < s1_n0 && s2_n1 > s1_n1 {
            // Segment 1 is completely contained by segment 2
            meet_distance = 0;
        }
        if meet_distance < std::i32::MAX {
            if horizontal1 {
                if seg1.x1 > seg1.x0 {
                    return Some(xy(seg1.x0 + meet_distance, seg1.y0));
                } else {
                    return Some(xy(seg1.x0 - meet_distance, seg1.y0));
                }
            } else {
                if seg1.y1 > seg1.y0 {
                    return Some(xy(seg1.x0, seg1.y0 + meet_distance));
                } else {
                    return Some(xy(seg1.x0, seg1.y0 - meet_distance));
                }
            }
        } else {
            return None
        }
    } // end if parallel

    // Now we are handling only perpendicular lines
    let seg1_xinterval = [min(seg1.x0, seg1.x1), max(seg1.x0, seg1.x1)];
    let seg1_yinterval = [min(seg1.y0, seg1.y1), max(seg1.y0, seg1.y1)];
    if seg2.x0 == seg2.x1 && seg2.x0 >= seg1_xinterval[0] && seg2.x0 <= seg1_xinterval[1] {
        let seg2_yinterval = [min(seg2.y0, seg2.y1), max(seg2.y0, seg2.y1)];
        // We know seg1.y0 == seg1.y1
        if seg1.y0 >= seg2_yinterval[0] && seg1.y0 <= seg2_yinterval[1] {
            // we have an intersection
            return Some(xy(seg2.x0, seg1.y0));
        }
    } else if seg2.y0 >= seg1_yinterval[0] && seg2.y0 <= seg1_yinterval[1] {
        let seg2_xinterval = [min(seg2.x0, seg2.x1), max(seg2.x0, seg2.x1)];
        if seg1.x0 >= seg2_xinterval[0] && seg1.x0 <= seg2_xinterval[1] {
            return Some(xy(seg1.x0, seg2.y0));
        }
    }
    return None
}

fn part2(lines: &Vec<Vec<Step>>) -> i32 {
    let segments0 = line_to_segments(&lines[0]);
    let segments1 = line_to_segments(&lines[1]);

    let mut shortest = std::i32::MAX;
    for seg0 in &segments0 {
        for seg1 in &segments1 {
            let loc = find_intersect(seg0, seg1);
            if loc.is_none() {
                continue;
            }
  
            let loc = loc.unwrap();
            // Exclude the origin
            if loc == xy(0, 0) {
                continue;
            }
            let score = seg0.d + loc.manhattan(xy(seg0.x0, seg0.y0)) +
                        seg1.d + loc.manhattan(xy(seg1.x0, seg1.y0));

            shortest = min(score, shortest);
        }
    }
    return shortest;
}


fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    println!("Reading from {}", opt.input);
    let lines = read_data_2d::<Step>(opt.input, ",").unwrap();

    // Part 1 is very memory inefficient, allocating a potentially very large 
    // grid in which most of the cells aren't even used. Part 2 forgoes this and 
    // just compares segments for overlap. 
    if opt.part1 {
        // Empirically, this was found ot be the size of grid required. Go ahead and 
        // initialize it to speed up later runs
        let mut grid = Grid::<bool>::new(-20224, -11136, 23552, 14592, None);
        let mut p = xy(0, 0);
        grid.set(&p, Some(true));
        
        for step in &lines[0] {
            println!("Moving {:?}", step);
            for _i in 0..step.length {
                p = p.go_one(step.dir);
                grid.set(&p, Some(true));
            }
        }

        let mut intersects = Vec::<Location>::new();
        let mut closest_distance = 1000000000; // arbitrary large number/psueudo IntMax
        p = xy(0, 0);
        for step in &lines[1] {
            println!("Moving {:?} from {:?}", step, p);
            for _i in 0..step.length {
                p = p.go_one(step.dir);
                match grid.get(&p) {
                    None => continue,
                    Some(_flag) => {
                        intersects.push(p);
                        let d = p.manhattan(xy(0, 0));
                        if d  < closest_distance {
                            closest_distance = d;
                        }
                    }
                }
            }
        }

        println!("Closest intersection: {:?}", closest_distance);
        
    } else {
        // Part 2
        println!("Running part2");
        let score = part2(&lines);
        println!("Closest intersect: {}", score);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_overlap() {
        #[derive(Debug)]
        struct Vector(Segment, Segment, Location);

        let vectors = [
            Vector(
                Segment{x0: 2, y0: 0, x1: 10, y1: 0, d: 0}, 
                Segment{x0: 8, y0: 0, x1: 3, y1: 0, d: 0},
                Location{x: 3, y: 0}),
            Vector(
                Segment{x0: 0, y0: -1, x1: 0, y1: -10, d: 0},
                Segment{x0: 0, y0: -1, x1: 0, y1: 5, d: 0},
                Location{x: 0, y: -1}),
            Vector(
                Segment{x0: -5, y0: 0, x1: 5, y1: 0, d: 0},
                Segment{x0: 0, y0: -5, x1: 0, y1: 5, d: 0},
                Location{x: 0, y: 0}),
            Vector(
                Segment{x0: 0, y0: 4, x1: 5, y1: 4, d: 0},
                Segment{x0: 5, y0: -5, x1: 5, y1: 5, d: 0},
                Location{x: 5, y: 4}),
        ];
        for v in &vectors {
            let y = find_intersect(&v.0, &v.1);
            assert!(y.is_some(), "None for case {:?}", v);
            assert_eq!(y.unwrap(), v.2);
        }
    }

    #[test]
    fn test_part2() {
        struct Vector(&'static str, i32);
        let vectors = [
            Vector(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                U62,R66,U55,R34,D71,R55,D58,R83", 610),
            Vector(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n \
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 410),
        ];

        for v in &vectors {
            let data = read_data_2d_str::<Step>(v.0.to_string(), ",").unwrap();
            let y = part2(&data);
            assert_eq!(y, v.1);
        }
    }
}