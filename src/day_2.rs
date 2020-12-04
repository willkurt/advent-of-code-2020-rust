pub fn part_1_and_2() {
    let file_path = "./data/day_2.txt";
    let mut valid: usize = 0;
    let mut valid_v2: usize = 0;
    if let Ok(pw_data) = crate::util::read_lines(file_path) {
        for raw_record in pw_data {
            let pw_record = parse_pwrecord(raw_record.unwrap());
            if pw_record.is_valid() {
                valid += 1;
            }
            if pw_record.is_valid_v2() {
                valid_v2 += 1;
            }
        }
    }
    println!("{} valid pws", valid);
    println!("{} valid_v2 pws", valid_v2);
}

struct PWRecord {
    rule_char: char,
    min_count: usize,
    max_count: usize,
    password: String,
}

impl PWRecord {
    fn is_valid(&self) -> bool {
        let mut char_count: usize = 0;
        for c in self.password.chars() {
            if c == self.rule_char {
                char_count += 1;
            }
        }
        (char_count >= self.min_count) & (char_count <= self.max_count)
    }

    fn is_valid_v2(&self) -> bool {
        let pw_vec: Vec<char> = self.password.chars().collect();
        let val1 = pw_vec[self.min_count - 1] == self.rule_char;
        let val2 = pw_vec[self.max_count - 1] == self.rule_char;
        (val1 | val2) & (!(val1 & val2))
    }
}

// an example line.
// 12-16 t: vdtbdtxtttttrctttkt
// note: this isn't ideal because of the O(N) copy time for remove
// a better strategy would be to move an index and then return
// the rest of the string... will refactor later.
fn consume_until(mut raw_text: String, term_char: char) -> (String, String) {
    let mut parse_result = String::from("");
    let mut next = raw_text.remove(0);
    while next != term_char {
        parse_result.push(next);
        next = raw_text.remove(0);
    }
    (parse_result, raw_text)
}

fn parse_pwrecord(raw_text: String) -> PWRecord {
    let (min_s, raw_text) = consume_until(raw_text, '-');
    let (max_s, raw_text) = consume_until(raw_text, ' ');
    let (rule_s, raw_text) = consume_until(raw_text, ':');
    let (_skip, final_text) = consume_until(raw_text, ' ');
    PWRecord {
        rule_char: rule_s.chars().next().unwrap(),
        min_count: min_s.parse::<usize>().unwrap(),
        max_count: max_s.parse::<usize>().unwrap(),
        password: final_text,
    }
}
