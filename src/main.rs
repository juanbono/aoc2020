use std::{env, io::Result};

mod day1;
mod day2;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let day = args
        .get(1)
        .expect("insert the day number to execute")
        .as_str();

    match day {
        "1" => day1::answer(),
        "2" => day2::answer(),
        _ => {
            println!("wrong day number");
            Ok(())
        }
    }
}
