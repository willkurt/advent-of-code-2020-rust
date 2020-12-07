#[macro_use]
mod macros;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod util;

use std::collections::HashSet;

/*
   let filename = "./data/day_6.txt";
   if let Ok(lines) = crate::util::read_lines(filename) {
       for line in lines {
           if let Ok(line) = line {
           }
       }
   }
*/

fn main() {
    println!("Welcome Advent of Code 2020: Rust!");
    println!("Day 1");
    day_1::part_1();
    day_1::part_2();
    println!("Day 2");
    day_2::part_1_and_2();
    println!("Day 3");
    day_3::part_1();
    day_3::part_2();
    println!("Day 4");
    //part 1 just checks for the existence of each field.
    day_4::part_2();
    //day 5
    println!("Day 5");
    day_5::part_1();
    day_5::part_2();
    println!("Day 6");
    part_1();
}

pub fn part_1() {
    let mut line_var = String::from("");
    let mut customs_checks = HashSet::<char>::new();
    let mut total = 0;
    with_read_lines!(
        "./data/day_6.txt", //file name
        line_var,           // variable used to capture line output
        {
            if line_var == "" {
                total += customs_checks.len();
                customs_checks.drain();
            } else {
                for c in line_var.chars() {
                    customs_checks.insert(c);
                }
            }
        },
        {
            total += customs_checks.len();
            println!("{} total checks", total);
        }
    );
}
