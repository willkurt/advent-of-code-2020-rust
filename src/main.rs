mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod util;

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
}
