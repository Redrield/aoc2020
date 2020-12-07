use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PassportElement {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColour,
    EyeColour,
    PassportId,
    CountryId,
}

impl PassportElement {
    const fn required_values() -> [PassportElement; 7] {
        [Self::BirthYear, Self::ExpirationYear, Self::EyeColour, Self::HairColour, Self::IssueYear, Self::Height, Self::PassportId]
    }

    fn from_name(n: &str) -> PassportElement {
        match n {
            "byr" => Self::BirthYear,
            "iyr" => Self::IssueYear,
            "eyr" => Self::ExpirationYear,
            "hgt" => Self::Height,
            "hcl" => Self::HairColour,
            "ecl" => Self::EyeColour,
            "pid" => Self::PassportId,
            "cid" => Self::CountryId,
            _ => unreachable!()
        }
    }
}

pub fn main() {
    let contents = std::fs::read_to_string("day4").unwrap().lines().map(ToString::to_string).collect::<Vec<String>>();

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &Vec<String>) {
    let mut passport: HashMap<PassportElement, String> = HashMap::new();
    let mut valid = 0u64;

    for line in contents {
        if line.as_str() == "" {
            if PassportElement::required_values().iter().all(|k| passport.contains_key(&k)) {
                valid += 1;
            }

            passport.clear();
            continue;
        }

        for elem in line.split_whitespace() {
            passport.insert(PassportElement::from_name(&elem[..3]), elem[4..].to_string());
        }
    }

    println!("{}", valid);
}

fn part2(contents: &Vec<String>) {
    let mut passport: HashMap<PassportElement, String> = HashMap::new();
    let mut valid = 0u64;

    let year_re: Regex = Regex::new(r#"^\d{4}$"#).unwrap();
    let height_re: Regex = Regex::new(r#"^\d+(in|cm)$"#).unwrap();
    let hcl_re: Regex = Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap();
    let ecl_re: Regex = Regex::new(r#"^(amb|blu|brn|gry|grn|hzl|oth)$"#).unwrap();
    let pid_re: Regex = Regex::new(r#"^\d{9}$"#).unwrap();

    for line in contents {
        if line.as_str() == "" {
            if PassportElement::required_values().iter().all(|k| passport.contains_key(&k)) {
                valid += 1;
            }

            passport.clear();
            continue;
        }

        for word in line.split_whitespace() {
            let elem = PassportElement::from_name(&word[..3]);
            let res = word[4..].to_string();
            match elem {
                PassportElement::BirthYear => {
                    if year_re.is_match(&res) {
                        let yr = res.parse::<u32>().unwrap();
                        if yr >= 1920 && yr <= 2002 {
                            passport.insert(elem, res);
                        }
                    }
                }
                PassportElement::IssueYear => {
                    if year_re.is_match(&res) {
                        let yr = res.parse::<u32>().unwrap();
                        if yr >= 2010 && yr <= 2020 {
                            passport.insert(elem, res);
                        }
                    }
                }
                PassportElement::ExpirationYear => {
                    if year_re.is_match(&res) {
                        let yr = res.parse::<u32>().unwrap();
                        if yr >= 2020 && yr <= 2030 {
                            passport.insert(elem, res);
                        }
                    }
                }
                PassportElement::Height => {
                    if height_re.is_match(&res) {
                        let hgt = res[..res.len() - 2].parse::<u32>().unwrap();
                        if res.contains("cm") {
                            if hgt >= 150 && hgt <= 193 {
                                passport.insert(elem, res);
                            }
                        } else if res.contains("in") {
                            if hgt >= 59 && hgt <= 76 {
                                passport.insert(elem, res);
                            }
                        }
                    }
                }
                PassportElement::HairColour => {
                    if hcl_re.is_match(&res) {
                        passport.insert(elem, res);
                    }
                }
                PassportElement::EyeColour => {
                    if ecl_re.is_match(&res) {
                        passport.insert(elem, res);
                    }
                }
                PassportElement::PassportId => {
                    if pid_re.is_match(&res) {
                        passport.insert(elem, res);
                    }
                }
                _ => {
                    passport.insert(elem, res);
                },
            }
        }
    }

    println!("{}", valid);
}