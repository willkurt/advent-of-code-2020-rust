use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

pub fn part_1() {
    let file_path = "./data/day_1.txt";
    let numbers = load_numbers(file_path);
    let target: i32 = 2020;
    'outer: for val1 in &numbers {
        for val2 in &numbers {
            if val1 + val2 == target {
                println!("{} * {} = {}", val1, val2, val1 * val2);
                break 'outer;
            }
        }
    }
}

// a bit lazy with this, but why not!
pub fn part_2() {
    let file_path = "./data/day_1.txt";
    let numbers = load_numbers(file_path);
    let target: i32 = 2020;
    'outer: for val1 in &numbers {
        for val2 in &numbers {
            for val3 in &numbers {
                if val1 + val2 + val3 == target {
                    println!("{} * {} * {} = {}", val1, val2, val3, val1 * val2 * val3);
                    break 'outer;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_numbers(filename: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(result) = line {
                let i_result: i32 = result.parse().expect("Failed to parse int!");
                numbers.push(i_result);
            }
        }
    }
    numbers
}
