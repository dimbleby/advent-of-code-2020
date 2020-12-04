use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Default)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn add_entry(&mut self, key: String, value: String) {
        self.fields.insert(key, value);
    }

    fn is_fully_valid(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }

        self.fields
            .iter()
            .all(|(key, value)| Self::validate_field(key, value))
    }

    fn has_required_fields(&self) -> bool {
        let required = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required.iter().all(|&key| self.fields.contains_key(key))
    }

    fn validate_field(key: &str, value: &str) -> bool {
        match key {
            "byr" => value
                .parse::<u32>()
                .map_or(false, |y| 1920 <= y && y <= 2002),
            "iyr" => value
                .parse::<u32>()
                .map_or(false, |y| 2010 <= y && y <= 2020),
            "eyr" => value
                .parse::<u32>()
                .map_or(false, |y| 2020 <= y && y <= 2030),
            "hgt" => {
                if let Some(cm) = value.strip_suffix("cm") {
                    cm.parse::<u32>().map_or(false, |cm| 150 <= cm && cm <= 193)
                } else if let Some(inches) = value.strip_suffix("in") {
                    inches
                        .parse::<u32>()
                        .map_or(false, |inches| 59 <= inches && inches <= 76)
                } else {
                    false
                }
            }
            "hcl" => {
                lazy_static! {
                    static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                }
                HCL.is_match(value)
            }
            "ecl" => {
                let valid = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                valid.contains(&value)
            }
            "pid" => {
                lazy_static! {
                    static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
                }
                PID.is_match(value)
            }
            "cid" => true,
            _ => panic!("unexpected field"),
        }
    }
}

pub(crate) fn day04() {
    let input = File::open("data/day04.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let lines = buffered.lines().map(|line| line.unwrap());

    let mut passports: Vec<Passport> = vec![];
    let mut passport = Passport::default();
    for line in lines {
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::default();
            continue;
        }

        let pairs = line.split(' ');
        for pair in pairs {
            let mut key_value = pair.split(':');
            let key = key_value.next().expect("no key");
            let value = key_value.next().expect("no value");
            passport.add_entry(key.to_owned(), value.to_owned());
        }
    }
    passports.push(passport);

    let valid = passports.iter().filter(|p| p.has_required_fields()).count();
    println!("Part one answer is {}", valid);

    let valid = passports.iter().filter(|p| p.is_fully_valid()).count();
    println!("Part two answer is {}", valid);
}
