
use structopt::StructOpt;
use aoc2019::StandardOptions;

const CODESIZE: usize = 6;

struct Code {
    digits: [i32; CODESIZE]
}

impl Code {
    pub fn new(value: u32) -> Code {
        let mut digits = [0i32; CODESIZE];
        for i in 0..CODESIZE {
            digits[i] = (value as i32 / 10i32.pow(i as u32)) % 10;
        }
        Code{digits}
    }

    pub fn inc(&mut self) {
        for i in 0..CODESIZE {
            let carry = self.digits[i] == 9;
            self.digits[i] = (self.digits[i] + 1) % 10;
            if !carry {
                break;
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut double = false;
        for i in (0..CODESIZE-1).rev() {
            if self.digits[i] == self.digits[i+1] {
                double = true;
            }
            if self.digits[i] < self.digits[i+1] {
                return false; // More significant digits must always be less than less significant
            }
        }
        return double;
    }

    pub fn is_valid_for_part2(&self) -> bool {
        let mut double = false;
        let mut repeat_count = 1;
        for i in (0..CODESIZE-1).rev() {
            if self.digits[i] == self.digits[i+1] {
                repeat_count += 1;
                if repeat_count == 2 {
                    if i == 0 || self.digits[i] != self.digits[i-1] {
                        double = true;
                    }
                }
            } else {
                repeat_count = 1
            }
            if self.digits[i] < self.digits[i+1] {
                return false; // More significant digits must always be less than less significant
            }
        }
        return double;
    }
}


fn part1(range_start: u32, range_end: u32 ) -> i32 {
    let mut code = Code::new(range_start);
    let mut valid_codes = 0;
    let mut steps = range_end - range_start + 1;
    while steps > 0 {
        if code.is_valid() {
            valid_codes += 1;
        }
        code.inc();
        steps -= 1;
    }
    valid_codes
}

fn part2(range_start: u32, range_end: u32 ) -> i32 {
    let mut code = Code::new(range_start);
    let mut valid_codes = 0;
    let mut steps = range_end - range_start + 1;
    while steps > 0 {
        if code.is_valid_for_part2() {
            valid_codes += 1;
        }
        code.inc();
        steps -= 1;
    }
    valid_codes
}

fn main() {

    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    const RANGE_START:u32 = 109165;
    const RANGE_END:u32 = 576723;
    // Part 1 is very memory inefficient, allocating a potentially very large 
    // grid in which most of the cells aren't even used. Part 2 forgoes this and 
    // just compares segments for overlap. 
    if opt.part1 {
        println!("Running part 1");
        let count = part1(RANGE_START, RANGE_END);
        println!("Count: {}", count);
    } else {
        println!("Running part 2");
        let count = part2(RANGE_START, RANGE_END);
        println!("Count: {}", count);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_code_new() {
        let code = Code::new(123456);
        assert_eq!(code.digits[0], 6);
        assert_eq!(code.digits[1], 5);
        assert_eq!(code.digits[2], 4);
        assert_eq!(code.digits[3], 3);
        assert_eq!(code.digits[4], 2);
        assert_eq!(code.digits[5], 1);
    }
    #[test]
    fn test_is_valid() {
        assert!( Code::new(112345).is_valid());
        assert!(!Code::new(123456).is_valid());
        assert!(!Code::new(112343).is_valid());
    }

    #[test]
    fn test_is_valid_for_part2() {
        assert!( Code::new(112345).is_valid_for_part2());
        assert!(!Code::new(111345).is_valid_for_part2());
        assert!( Code::new(111144).is_valid_for_part2());
    }
}