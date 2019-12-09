// Today's problem was relatively simple, but wow rust memory rules make it complicated. 
// First, you have to implicitly handle your reference counting with Rc; OK. But then, 
// if you ever want to get a mutable reference to the data, you have to further wrap
// it in a RefCell, to handle run-time checked borrowing. So yay, I know I won't have 
// memory errors, but it sure imposes a lot of work on me, makes this code harder to
// read, and this really doesn't feel like a good trade-off for this use case. 
//
// I really hate having to parse through all of the borrow, borrow_mut, clone, unwrap
// and such. 

use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Error;
use structopt::StructOpt;

use aoc2019::io::{read_data};
use aoc2019::StandardOptions;

struct Map {
    node_map: HashMap<String, Rc<RefCell<Node>>>,
}

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    name: String,
    orbits: i32,
}

impl Node {
    pub fn score(&mut self) -> i32 {
        // If we've already computed the value for this score, use cached value
        // otherwise, compute it recursively
        if self.orbits < 0 {
            match &self.parent {
                None => self.orbits = 0,
                Some(p) => self.orbits = p.borrow_mut().score() + 1,
            }
        }
        self.orbits
    }
}

// Represent orbit relationship where body orbits center
#[derive(Debug)]
struct Edge {
    center: String,
    body: String
}

impl FromStr for Edge {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let names: Vec::<&str> = s.split(")").collect();

        if names.len() != 2 {
            panic!("Bad string: {}", s);
        }

        Ok(Edge{center: names[0].to_string(), body: names[1].to_string()})
    }
}

fn build_map(edges: &Vec<Edge>) -> Map {
    let mut nodes = HashMap::<String, Rc<RefCell<Node>>>::new();
    for edge in edges {
        let parent: Rc<RefCell<Node>>;
        if nodes.contains_key(&edge.center) {
            parent = nodes.get(&edge.center).unwrap().clone();
        } else {
            parent = Rc::new(RefCell::new(Node{parent: None, name: edge.center.clone(), orbits: -1}));
            nodes.insert(edge.center.clone(), parent.clone());
        }
        if nodes.contains_key(&edge.body) {
            // The node was already created as a parent; just update it
            let mut new_node = nodes.get(&edge.body).unwrap().borrow_mut();
            new_node.parent = Some(parent.clone());
        } else {
            let new_node = Node{parent: Some(parent.clone()), name: edge.body.clone(), orbits: -1};
            nodes.insert(edge.body.clone(), Rc::new(RefCell::new(new_node)));
        }
    }
    Map{node_map: nodes}
}

fn part1(edges: &Vec<Edge>) -> i32 {
    let mut sum = 0;
    let map = build_map(edges);

    for node in map.node_map.values() {
        sum += node.borrow_mut().score();
    }
    
    sum
}

fn part2(edges: &Vec<Edge>) -> i32 {
    let map = build_map(edges);
    let mut santa_parents = Vec::<String>::new();
    let mut node: Rc<RefCell<Node>> = map.node_map.get("SAN").unwrap().clone();
    loop {
        santa_parents.push(node.borrow().name.clone());
        let parent = node.borrow().parent.clone();
        match &parent {
            None => break,
            Some(p) => node = p.clone(),
        }
    }

    // Find the first ancestor that is also an ancestor of santa
    node = map.node_map.get("YOU").unwrap().clone();
    let mut distance = 0;
    loop {
        if santa_parents.contains(&node.borrow().name.clone()) {
            break;
        }
        distance += 1;
        // This should be impossible, since all nodes are supposed to share the common root
        if !node.borrow().parent.is_some() {
            panic!("No path found to santa!");
        }
        let parent = (&node.borrow().parent).as_ref().unwrap().clone();
        node = parent;
    }

    let santa_position = santa_parents.iter().position(|n| *n == node.borrow().name).unwrap() as i32;
    santa_position + distance - 2
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    let edges = read_data::<Edge>(opt.input, "\n").unwrap();
    if opt.part1 {
        let score = part1(&edges);
        println!("Score: {}", score);
    } else {
        let score = part2(&edges);
        println!("Traversal distance: {}", score);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use aoc2019::io::read_data_str;
    #[test]
    fn test_part1() {
        let input = "
            COM)B\n\
            B)C\n\
            C)D\n\
            E)F\n\
            D)E\n\
            B)G\n\
            G)H\n\
            D)I\n\
            E)J\n\
            J)K\n\
            K)L".to_string();

        let expected_result = 42;
        let input = read_data_str::<Edge>(input, "\n").unwrap();
        let result = part1(&input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input = "
            COM)B\n\
            B)C\n\
            C)D\n\
            D)E\n\
            E)F\n\
            B)G\n\
            G)H\n\
            D)I\n\
            E)J\n\
            J)K\n\
            K)L\n\
            K)YOU\n\
            I)SAN".to_string();
        let expected_result = 4;
        let input = read_data_str::<Edge>(input, "\n").unwrap();
        let result = part2(&input);
        assert_eq!(result, expected_result);
    }
}