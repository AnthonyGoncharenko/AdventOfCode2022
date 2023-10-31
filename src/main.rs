pub mod days;
use std::env;

#[derive(Debug)]
pub enum AdventError {
    MissingArg(String),
    ParseInt(String),
    NotAdventNumber(String),
}

fn main() -> Result<(), AdventError> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if let Some(day) = args.get(1) {
        let parsed_day = match day.parse::<i32>() {
            Ok(t) => t,
            Err(e) => return Err(AdventError::ParseInt(e.to_string())),
        };
        if let Err(e) = call_day(parsed_day) {
            return Err(e);
        }
    } else {
        return Err(AdventError::MissingArg(
            "Usage :: cargo run [day]".to_string(),
        ));
    }

    Ok(())
}

fn call_day(day: i32) -> Result<(), AdventError> {
    match day {
        1 => days::day_01::solution(),
        2 => days::day_02::solution(),
        3 => days::day_03::solution(),
        4 => days::day_04::solution(),
        // 5 => days::day5::solution(),
        // 6 => days::day6::solution(),
        // 7 => days::day7::solution(),
        // 8 => days::day8::solution(),
        // 9 => days::day9::solution(),
        // 10 => days::day10::solution(),
        e => Err(AdventError::NotAdventNumber(
            format!("This day: [{}], is not very Adventy!", e).to_string(),
        )),
    }
}
