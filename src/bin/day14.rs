use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Error};
use regex::Regex;
use structopt::StructOpt;

use aoc2019::StandardOptions;
use aoc2019::io::read_data;

#[derive(Debug, Clone)]
struct RecipeItem {
    units: i64,
    material: String,
}

#[derive(Debug, Clone)]
struct Recipe {
    input: Vec<RecipeItem>,
    output: RecipeItem
}


impl FromStr for Recipe {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let words: Vec<&str> = s.split("=>").collect();
        let input_str = words[0];
        let output_str = words[1];
        let mut input: Vec<RecipeItem> = Vec::new();
        let mut output: RecipeItem;

        fn parse_item(s: &str) -> RecipeItem {
            let re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
            let caps = re.captures(s).unwrap();
            let units = caps[1].parse::<i64>().unwrap();
            let material = &caps[2];
            RecipeItem{units, material: material.to_string()}
        }
        for item_str in input_str.split(",") {
            input.push(parse_item(item_str));
        }

        output = parse_item(output_str);
        Ok(Recipe{input, output})
    }
}

fn find_recipe(recipes: &Vec<Recipe>, name: &str) -> Recipe {
    for r in recipes {
        if r.output.material == name {
            return r.clone();
        }
    }
    panic!("Couldn't find recipe for {}", name);
}

fn find_ore(recipes: &Vec<Recipe>, excess: &mut HashMap<String, i64>, material: &str, qty: i64) -> i64 {
    let r = find_recipe(&recipes, material);

    let mut qty = qty;

    // Get any available excess off the stash
    if excess.contains_key(material) {
        let excess_mat = excess.get_mut(material).unwrap();
        let used_excess: i64 = min(*excess_mat, qty);
        qty -= used_excess;
        *excess_mat -= used_excess;
    }

    if qty == 0 {
        return 0;
    }

    // Figure out how many iterations of the recipe will be neededd 
    let n = (qty as f64 / r.output.units as f64).ceil() as i64;

    let excess_output = n * r.output.units - qty;
    if !excess.contains_key(material) {
        excess.insert(material.to_string(), 0);
    }
    *excess.get_mut(material).unwrap() += excess_output;

    let mut ore_required: i64 = 0;
    for input in r.input {
        if input.material == "ORE" {
            ore_required += n * input.units;
        } else {
            let ore = find_ore(recipes, excess, &input.material, n * input.units);
            ore_required += ore;
        }
    }

    return ore_required;
}

fn part1(recipes: &Vec<Recipe>) -> i32 {
    // Keep track of leftovers from previous reactions in case we can re-use them in later reactions
    let mut excess: HashMap<String, i64> = HashMap::new();

    let ore_required = find_ore(&recipes, &mut excess, "FUEL", 1);
    ore_required as i32
}

fn part2(recipes: &Vec<Recipe>, part1_answer: i32) -> i64 {
    // Use the  answer from part 1 --how much ORE to make one FUEL --
    // to figure out the minimum amount of fuel we'll get with 1trillion
    // Maybe there's a clever way, but I'm just going to brute force this
    const TRILLION: i64 = 1000 * 1000 * 1000 * 1000;
    let mut n: i64 = TRILLION / part1_answer as i64;
    
    // There's a lot to be gained here by doing more levels of granularity, 
    // or adapting step size based on gradient. But I don't plan to run this ever again...
    const COARSE_STEP: i64 = 1000;
    loop {
        let mut excess: HashMap<String, i64> = HashMap::new();
        n += COARSE_STEP;
        
        let ore = find_ore(&recipes, &mut excess, "FUEL", n);
        //println!("n={}, ore={}", n, ore);
        if ore > TRILLION {
            break;
        }
    }
    n -= COARSE_STEP;

    loop {
        let mut excess: HashMap<String, i64> = HashMap::new();
        let ore = find_ore(&recipes, &mut excess,"FUEL", n);
        if ore > TRILLION {
            return n-1;
        }
        n += 1;
    }
}

fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    let recipes: Vec<Recipe> = read_data(opt.input, "\n").unwrap();
    if opt.part1 {
        let input_ore = part1(&recipes);
        println!("Required ORE for 1 FUEL: {}", input_ore);
    } else {
        let part1_answer = part1(&recipes);
        let part2_answer = part2(&recipes, part1_answer);
        println!("Total Fuel: {}", part2_answer);
    }
    
}

#[cfg(test)]
mod tests {
    use crate::*;
    use aoc2019::io::read_data_str;
    #[test]
    fn test_ex1() {
        let input = "157 ORE => 5 NZVS\n\
                    165 ORE => 6 DCFZ\n\
                    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
                    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
                    179 ORE => 7 PSHF\n\
                    177 ORE => 5 HKGWZ\n\
                    7 DCFZ, 7 PSHF => 2 XJWVT\n\
                    165 ORE => 2 GPVTF\n\
                    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string();
        let recipes = read_data_str(input, "\n").unwrap();
        let part1_ore_required = part1(&recipes);
        let part2_max_fuel = part2(&recipes, part1_ore_required);
        assert_eq!(part1_ore_required, 13312);
        assert_eq!(part2_max_fuel, 82892753);
    }

    #[test]
    fn test_ex3() {
        let input = "171 ORE => 8 CNZTR\n\
                    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
                    114 ORE => 4 BHXH\n\
                    14 VRPVC => 6 BMBT\n\
                    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
                    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
                    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
                    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
                    5 BMBT => 4 WPTQ\n\
                    189 ORE => 9 KTJDG\n\
                    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
                    12 VRPVC, 27 CNZTR => 2 XDBXC\n\
                    15 KTJDG, 12 BHXH => 5 XCVML\n\
                    3 BHXH, 2 VRPVC => 7 MZWV\n\
                    121 ORE => 7 VRPVC\n\
                    7 XCVML => 6 RJRHP\n\
                    5 BHXH, 4 VRPVC => 5 LTCX".to_string();
        let recipes = read_data_str(input, "\n").unwrap();
        let part1_ore_required = part1(&recipes);
        assert_eq!(part1_ore_required, 2210736);
        let part2_max_fuel = part2(&recipes, part1_ore_required);
        assert_eq!(part2_max_fuel, 460664);
    }
}