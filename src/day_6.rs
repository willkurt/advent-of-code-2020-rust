use std::collections::HashSet;
pub fn part_2() {
    let mut line_var = String::from("");
    let mut customs_checks = HashSet::<char>::new();
    let mut total = 0;
    let mut first_person = true;
    with_read_lines!(
        "./data/day_6.txt", //file name
        line_var,           // variable used to capture line output
        {
            if (line_var == "") & (!first_person) {
                total += customs_checks.len();
                customs_checks.drain();
                first_person = true;
            } else {
                if first_person {
                    for c in line_var.chars() {
                        customs_checks.insert(c);
                    }
                    first_person = false
                } else {
                    let mut next_checks = HashSet::<char>::new();
                    for c in line_var.chars() {
                        next_checks.insert(c);
                    }
                    customs_checks = customs_checks.intersection(&next_checks).copied().collect();
                }
            }
        },
        {
            total += customs_checks.len();
            println!("{} total checks", total);
        }
    );
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
