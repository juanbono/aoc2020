extern crate serde;
use serde_scan::{scan, ScanError};
use std::io::prelude::*;
use std::{fs::File, str::FromStr};

enum Scheme {
    Old,
    New,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Policy {
    fst: usize,
    snd: usize,
    letter: char,
}

impl Policy {
    fn is_valid(&self, password: String, scheme: Scheme) -> bool {
        match scheme {
            Scheme::Old => {
                let occurrences: usize = password
                    .chars()
                    .filter(|chr| self.letter == *chr)
                    .collect::<Vec<char>>()
                    .len();
                let above_minimum = self.fst <= occurrences;
                let below_maximum = occurrences <= self.snd;
                above_minimum && below_maximum
            }
            Scheme::New => {
                let matches_first = password
                    .char_indices()
                    .find(|&(pos, chr)| (pos + 1) == self.fst && chr == self.letter)
                    .is_some();

                let matches_second = password
                    .char_indices()
                    .find(|&(pos, chr)| (pos + 1) == self.snd && chr == self.letter)
                    .is_some();

                matches_first ^ matches_second
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    policy: Policy,
    password: String,
}

impl Line {
    fn is_valid(&self, scheme: Scheme) -> bool {
        self.policy.is_valid(self.password.clone(), scheme)
    }
}

impl FromStr for Line {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field: (usize, usize, char, &str) = scan!("{}-{} {}: {}" <- s)?;
        let (fst, snd, letter, password) = field;
        let policy = Policy { fst, snd, letter };
        Ok(Line {
            policy,
            password: String::from(password),
        })
    }
}

fn parse_input(input: String) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(|line| line.parse())
        .filter_map(|res| res.ok())
        .collect()
}

pub fn answer() -> std::io::Result<()> {
    let mut file = File::open("src/inputs/day2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = parse_input(contents);

    let valid_passwords = lines
        .iter()
        .filter(|line| line.is_valid(Scheme::Old))
        .count();
    println!("valid passwords (old scheme): {:?}", valid_passwords);

    let valid_passwords2 = lines
        .iter()
        .filter(|line| line.is_valid(Scheme::New))
        .count();
    println!("valid passwords (new scheme): {:?}", valid_passwords2);
    Ok(())
}

#[test]
fn test_line_parsing() {
    let line = "5-6 l: llllkl";
    let expected_policy = Policy {
        fst: 5,
        snd: 6,
        letter: 'l',
    };
    let expected_line = Line {
        policy: expected_policy,
        password: String::from("llllkl"),
    };

    assert_eq!(expected_line, line.parse().unwrap());
}

#[test]
fn test_line_is_valid_old() {
    let line1: Line = "1-3 a: abcde".parse().unwrap();
    let line2: Line = "1-3 b: cdefg".parse().unwrap();
    let line3: Line = "2-9 c: ccccccccc".parse().unwrap();

    assert!(line1.is_valid(Scheme::Old));
    assert!(!line2.is_valid(Scheme::Old));
    assert!(line3.is_valid(Scheme::Old));
}

#[test]
fn test_line_is_valid_new() {
    let line1: Line = "1-3 a: abcde".parse().unwrap();
    let line2: Line = "1-3 b: cdefg".parse().unwrap();
    let line3: Line = "2-9 c: ccccccccc".parse().unwrap();

    assert!(line1.is_valid(Scheme::New));
    assert!(!line2.is_valid(Scheme::New));
    assert!(!line3.is_valid(Scheme::New));
}
