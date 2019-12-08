use std::fs;
use std::str::FromStr;
use anyhow::Result;

/// Read data from a file, splitting first by line, and then by `split_string`, and 
/// converting each element to type T
pub fn read_data_2d<T: FromStr>(filepath: String, split_string: &str) -> Result<Vec<Vec<T>>> {
    let content = fs::read_to_string(filepath)?;
    read_data_2d_str(content, split_string)
} 

pub fn read_data_2d_str<T: FromStr>(content: String, split_string: &str) -> Result<Vec<Vec<T>>> {
    let text_lines = content.split("\n");
    let data = text_lines.map( |s| {
        s.trim().split(split_string).filter_map( |s| {
            if s.len() > 0 { 
                let result = s.parse::<T>();
                match result {
                    Ok(parsed) => Some(parsed),
                    Err(_e) => panic!("Bad input string"),
                }
            } else { 
                None 
            }
        }).collect()
    }).collect();
    Ok(data)
} 