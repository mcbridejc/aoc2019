use std::fs;

use structopt::StructOpt;
use aoc2019::StandardOptions;
use anyhow::Result;

struct Layer {
    data: Vec<i32>,
    width: i32,
}

impl Layer {
    pub fn get(&self, row: i32, col: i32) -> i32 {
        self.data[(row*self.width + col) as usize]
    }

    pub fn height(&self) -> i32 {
        self.data.len() as i32 / self.width
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_string();
        for row in 0..self.height() {
            for col in 0..self.width {
                s.push_str(&self.get(row, col).to_string());
            }
            s.push_str("\n");
        }
        s
    }

    pub fn to_binary(&self) -> String {
        let mut s = "".to_string();
        for row in 0..self.height() {
            for col in 0..self.width {
                if self.get(row, col) == 1 {
                    s.push_str("*");
                } else {
                    s.push_str(" ");
                }
            }
            s.push_str("\n");
        }
        s
    }
}

fn flatten_image(input: &Vec<Layer>) -> Layer {
    let mut out = Layer{data: vec![2; input[0].data.len()], width: input[0].width};
    for l in input { 
        for i in 0..l.data.len() {
            if out.data[i] == 2 {
                out.data[i] = l.data[i];
            }
        }
    }
    out
}

fn read_space_image(filepath: String, width: i32, height: i32) -> Result<Vec<Layer>> {
    let content = fs::read_to_string(filepath)?;
    read_space_image_from_str(&content, width, height)
}

fn read_space_image_from_str(data: &str, width: i32, height: i32) -> Result<Vec<Layer>> {
    let data = data.trim();
    let layer_size = (width * height) as usize;
    if data.len() % layer_size != 0 {
        panic!("Data length {} is invalid for image size {}x{}", data.len(), width, height);
    }
    let mut img = Vec::<Layer>::new();
    let mut layer_data = Vec::<i32>::new();
    for i in 0usize..data.len() {
        layer_data.push(data[i..i+1].parse::<i32>().unwrap());
        if (i+1) % layer_size == 0 {
            img.push(Layer{data: layer_data, width: width});
            layer_data = Vec::new();
        }
    }
    Ok(img)
}

fn part1(img: &Vec<Layer>) -> i32 {
    let mut layer_idx = -1;
    let mut min_count = std::i32::MAX;
    for (i, l) in img.iter().enumerate() { 
        let mut zero_count = 0;
        for p in &l.data {
            if *p == 0 {
                zero_count += 1;
            }
        }
        println!("Layer {} contains {} zeros", i, zero_count);
        if zero_count < min_count {
            min_count = zero_count;
            layer_idx = i as i32;
        }
    }

    let mut one_count = 0;
    let mut two_count = 0;    
    for p in &img[layer_idx as usize].data {
        if *p == 1 {
            one_count += 1;
        } else if *p == 2 {
            two_count += 1;
        }
    }

    println!("Layer {} has {} ones and {} twos", layer_idx, one_count, two_count);
    one_count * two_count
}

fn part2(img: &Vec<Layer>) {
    let flat = flatten_image(img);
    println!("Image: \n{}", flat.to_binary());
}


fn main() {
    let opt = StandardOptions::from_args();
    let _ = simple_logger::init();

    // Width and height are given in problem
    let width = 25;
    let height = 6; 

    let image = read_space_image(opt.input, width, height).unwrap();

    if opt.part1 {
        let result = part1(&image);
        println!("Answer: {}", result);
        println!("Layer 7: \n{}", image[7].to_string());
    } else {
        part2(&image);
        
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_read_layer() {
        let data = "123456789012";
        let image = read_space_image_from_str(&data, 3, 2).unwrap();
        assert_eq!(image.len(), 2);
        assert_eq!(image[0].get(0, 1), 2);
        assert_eq!(image[1].get(1, 2), 2);
        assert_eq!(image[1].get(0, 0), 7);
    }
}