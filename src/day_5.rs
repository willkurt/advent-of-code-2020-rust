// max seat id 1023, meaning an array of size 1024
pub fn part_2() {
    let mut seats: [bool; 1024] = [false; 1024];
    let mut line_var = String::from("");
    with_read_lines!(
        "./data/day_5.txt",
        line_var,
        {
            let line_v = line_var.chars().collect();
            let row_seat = parse_ticket_code(&line_v, 0, 0, 0);
            let seat_id = calc_seat_id(row_seat);
            seats[seat_id as usize] = true;
        },
        {
            for i in 40..(980) {
                if !seats[i] {
                    println!("{} is empty", i);
                }
            }
        }
    );
}

pub fn part_1() {
    let mut max_id: u32 = 0;
    let filename = "./data/day_5.txt";
    if let Ok(lines) = crate::util::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let line_v = line.chars().collect();
                let row_seat = parse_ticket_code(&line_v, 0, 0, 0);
                let seat_id = calc_seat_id(row_seat);
                if seat_id > max_id {
                    max_id = seat_id;
                }
            }
        }
    }
    println!("max is {}", max_id);
}

fn calc_seat_id(row_seat: (u32, u32)) -> u32 {
    row_seat.0 * 8 + row_seat.1
}

fn parse_ticket_code(code: &Vec<char>, i: u32, row: u32, seat: u32) -> (u32, u32) {
    let base: u32 = 2;
    let row_limit: u32 = 6;
    let seat_limit: u32 = 9;
    if (i as usize) == code.len() {
        return (row, seat);
    }
    match code[i as usize] {
        'B' => parse_ticket_code(code, i + 1, row + (base.pow(row_limit - i)), seat),
        'F' => parse_ticket_code(code, i + 1, row, seat),
        'L' => parse_ticket_code(code, i + 1, row, seat),
        'R' => parse_ticket_code(code, i + 1, row, seat + base.pow(seat_limit - i)),
        _ => (0, 0),
    }
}
