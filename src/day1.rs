use itertools::iproduct;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn parse_input(input: String) -> Vec<u32> {
    let mut nums = Vec::new();
    for line in input.lines() {
        if let Ok(num) = line.parse::<u32>() {
            nums.push(num)
        }
    }
    nums
}

pub fn find_2020_tuple(nums: Vec<u32>) -> Option<(u32, u32)> {
    let mut found = None;
    for (&x, &y) in iproduct!(nums.iter(), nums.iter()) {
        if x + y == 2020 {
            found = Some((x, y))
        }
    }
    found
}

pub fn find_2020_triple(nums: Vec<u32>) -> Option<(u32, u32, u32)> {
    let mut found = None;
    for (&x, &y, &z) in iproduct!(nums.iter(), nums.iter(), nums.iter()) {
        if x + y + z == 2020 {
            found = Some((x, y, z))
        }
    }
    found
}

pub fn answer() -> Result<()> {
    let mut file = File::open("src/inputs/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let nums = parse_input(contents);
    println!("Part 1\n=======");
    match find_2020_tuple(nums.clone()) {
        Some(t) => {
            println!("Found a tuple: {:?}", t);
            let (x, y) = t;
            println!("Answer: {:?}", x * y)
        }
        None => println!("something went wrong :("),
    }
    println!("Part 2\n=======");
    match find_2020_triple(nums) {
        Some(t) => {
            println!("Found a triple: {:?}", t);
            let (x, y, z) = t;
            println!("Answer: {:?}", x * y * z)
        }
        None => println!("something went wrong :("),
    }

    Ok(())
}
