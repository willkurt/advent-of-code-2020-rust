pub fn part_2() {
    let passports = load_passports("./data/day_4.txt");
    let mut ct = 0;
    for passport in passports {
        if passport.is_valid() {
            ct += 1;
        }
    }
}

fn load_passports(filename: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut cur_index: usize = 0;
    passports.push(new_passport());
    if let Ok(lines) = crate::util::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 {
                    passports.push(new_passport());
                    cur_index += 1;
                } else {
                    passports[cur_index].parse_line(line);
                }
            }
        }
    }
    passports
}

fn new_passport() -> Passport {
    Passport {
        cid: Err("none"),
        byr: Err("none"),
        iyr: Err("none"),
        eyr: Err("none"),
        hgt: Err("none"),
        hcl: Err("none"),
        ecl: Err("none"),
        pid: Err("none"),
    }
}

struct Passport {
    cid: Result<String, &'static str>,
    byr: Result<usize, &'static str>,
    iyr: Result<usize, &'static str>,
    eyr: Result<usize, &'static str>,
    hgt: Result<String, &'static str>,
    hcl: Result<String, &'static str>,
    ecl: Result<String, &'static str>,
    pid: Result<String, &'static str>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        match self {
            Passport {
                cid: _,
                byr: Ok(_),
                iyr: Ok(_),
                eyr: Ok(_),
                hgt: Ok(_),
                hcl: Ok(_),
                ecl: Ok(_),
                pid: Ok(_),
            } => true,
            _ => false,
        }
    }

    fn parse_line(&mut self, line: String) {
        let mut buff = String::from("");
        let mut key = String::from("");
        for c in line.chars() {
            match c {
                ':' => {
                    key = buff;
                    buff = String::from("");
                }
                ' ' | '\n' => {
                    self.str_set(key, buff);
                    key = String::from("");
                    buff = String::from("");
                }
                _ => buff.push(c),
            }
        }
        if (buff.len() > 0) & (key.len() > 0) {
            self.str_set(key, buff);
        }
    }

    fn str_set(&mut self, key: String, val: String) -> bool {
        match key.as_str() {
            "cid" => self.cid = Ok(val),
            "byr" => self.byr = parse_byr(val),
            "iyr" => self.iyr = parse_iyr(val),
            "eyr" => self.eyr = parse_eyr(val),
            "hgt" => self.hgt = parse_hgt(val),
            "hcl" => self.hcl = parse_hcl(val),
            "ecl" => self.ecl = parse_ecl(val),
            "pid" => self.pid = parse_pid(val),
            _ => return false,
        }
        true
    }
}

fn parse_restricted_year(val: String, low: usize, high: usize) -> Result<usize, &'static str> {
    let yr = val.parse::<usize>();
    match yr {
        Ok(year) => {
            if (year >= low) & (year <= high) {
                Ok(year)
            } else {
                Err("year out of bounds")
            }
        }
        _ => Err("bad year parse"),
    }
}

fn parse_byr(val: String) -> Result<usize, &'static str> {
    parse_restricted_year(val, 1920, 2002)
}

fn parse_iyr(val: String) -> Result<usize, &'static str> {
    parse_restricted_year(val, 2010, 2020)
}

fn parse_eyr(val: String) -> Result<usize, &'static str> {
    parse_restricted_year(val, 2020, 2030)
}

fn parse_regex(val: String, re: Regex) -> Result<String, &'static str> {
    if re.is_match(val.as_str()) {
        Ok(val)
    } else {
        Err("incorrect parse")
    }
}

fn parse_hgt(val: String) -> Result<String, &'static str> {
    let re = Regex::new(r"^[0-9]+(cm|in)$").unwrap();
    if let Ok(hgt_s) = parse_regex(val, re) {
        let unit = &hgt_s[(hgt_s.len() - 2)..hgt_s.len()];
        let measure_s = &hgt_s[0..(hgt_s.len() - 2)];
        let measure_i = measure_s.parse::<usize>().unwrap();
        if ((unit == "cm") & (measure_i <= 193) & (measure_i >= 150))
            | ((unit == "in") & (measure_i <= 76) & (measure_i >= 59))
        {
            Ok(hgt_s)
        } else {
            Err("invalid size")
        }
    } else {
        Err("parse failed")
    }
}

fn parse_hcl(val: String) -> Result<String, &'static str> {
    let re = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
    parse_regex(val, re)
}

fn parse_ecl(val: String) -> Result<String, &'static str> {
    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    parse_regex(val, re)
}

fn parse_pid(val: String) -> Result<String, &'static str> {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    parse_regex(val, re)
}
