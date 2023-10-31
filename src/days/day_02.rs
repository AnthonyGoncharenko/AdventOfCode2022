use core::panic;
use std::str::FromStr;

fn part_one(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let choices = line.split_ascii_whitespace().collect::<Vec<_>>();
        let opp_choice = match rps_strategy::OpponentStrategy::from_str(choices.first().unwrap()) {
            Ok(o) => o,
            Err(e) => {
                println!("{:#?}", e);
                panic!()
            }
        };
        let my_choice = match rps_strategy::MyStrategy::from_str(choices.get(1).unwrap()) {
            Ok(o) => o,
            Err(e) => {
                println!("{:#?}", e);
                panic!()
            }
        };

        let opp_choice = rps_strategy::Rps::from_opponent(opp_choice);
        let my_choice = rps_strategy::Rps::from_me(my_choice);
        score += match my_choice.cmp(&opp_choice) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        };
        score += i32::from(my_choice);
    }
    score
}
fn part_two(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let choices: Vec<&str> = line.split_ascii_whitespace().collect();
        let opp_choice = match rps_strategy::OpponentStrategy::from_str(choices.first().unwrap()) {
            Ok(o) => o,
            Err(e) => {
                println!("{:#?}", e);
                panic!()
            }
        };
        let my_choice = match rps_strategy::MyStrategy::from_str(choices.get(1).unwrap()) {
            Ok(o) => o,
            Err(e) => {
                println!("{:#?}", e);
                panic!()
            }
        };

        let opp_choice = rps_strategy::Rps::from_opponent(opp_choice);
        let my_choice = rps_strategy::Rps::from_me_truth(&my_choice, &opp_choice);
        score += match my_choice.cmp(&opp_choice) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        };
        score += i32::from(my_choice);
    }
    score
}

pub fn solution() -> Result<(), crate::AdventError> {
    let input = include_str!("./inputs/02.txt");

    println!("Part One's Score: [{}]", part_one(input));
    println!("Part Two's Score: [{}]", part_two(input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "A Y
B X
C Z";
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 15)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 12)
    }
}

mod rps_strategy {
    use std::{cmp::Ordering, str::FromStr};

    #[derive(PartialOrd, Eq, PartialEq)]
    pub enum Rps {
        Rock,
        Paper,
        Scissors,
    }
    pub enum OpponentStrategy {
        A,
        B,
        C,
    }

    pub enum MyStrategy {
        X,
        Y,
        Z,
    }

    impl Rps {
        pub fn from_opponent(opp_strat: OpponentStrategy) -> Rps {
            use OpponentStrategy as OS;
            match opp_strat {
                OS::A => Self::Rock,
                OS::B => Self::Paper,
                OS::C => Self::Scissors,
            }
        }
        pub fn from_me(my_strat: MyStrategy) -> Rps {
            use MyStrategy as MS;
            match my_strat {
                MS::X => Self::Rock,
                MS::Y => Self::Paper,
                MS::Z => Self::Scissors,
            }
        }
        pub fn from_me_truth(my_strat: &MyStrategy, opp_strat: &Rps) -> Rps {
            use MyStrategy as MS;
            match opp_strat {
                // Rock
                Self::Rock => match my_strat {
                    MS::X => Self::Scissors, // Lose
                    MS::Y => Self::Rock,     // Draw
                    MS::Z => Self::Paper,    // Win
                },
                // Paper
                Self::Paper => match my_strat {
                    MS::X => Self::Rock,
                    MS::Y => Self::Paper,
                    MS::Z => Self::Scissors,
                },
                // Scissors
                Self::Scissors => match my_strat {
                    MS::X => Self::Paper,
                    MS::Y => Self::Scissors,
                    MS::Z => Self::Rock,
                },
            }
        }
    }
    impl Ord for Rps {
        fn cmp(&self, other: &Self) -> Ordering {
            match self {
                Rps::Paper => match other {
                    Rps::Paper => Ordering::Equal,
                    Rps::Rock => Ordering::Greater,
                    Rps::Scissors => Ordering::Less,
                },
                Rps::Rock => match other {
                    Rps::Paper => Ordering::Less,
                    Rps::Rock => Ordering::Equal,
                    Rps::Scissors => Ordering::Greater,
                },
                Rps::Scissors => match other {
                    Rps::Paper => Ordering::Greater,
                    Rps::Rock => Ordering::Less,
                    Rps::Scissors => Ordering::Equal,
                },
            }
        }
    }
    impl From<Rps> for i32 {
        fn from(val: Rps) -> Self {
            match val {
                Rps::Rock => 1,
                Rps::Paper => 2,
                Rps::Scissors => 3,
            }
        }
    }

    #[derive(Debug)]
    pub struct InputError {
        pub message: String,
    }
    impl InputError {
        fn new(s: String) -> Self {
            InputError { message: s }
        }
    }
    impl FromStr for OpponentStrategy {
        type Err = InputError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use OpponentStrategy as OS;
            if s == "A" {
                Ok(OS::A)
            } else if s == "B" {
                Ok(OS::B)
            } else if s == "C" {
                Ok(OS::C)
            } else {
                Err(InputError::new(
                    "Invalid Input. Should be A, B, or C.".to_string(),
                ))
            }
        }
    }
    impl FromStr for MyStrategy {
        type Err = InputError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use MyStrategy as MS;
            if s == "X" {
                Ok(MS::X)
            } else if s == "Y" {
                Ok(MS::Y)
            } else if s == "Z" {
                Ok(MS::Z)
            } else {
                Err(InputError::new(
                    "Invalid Input. Should be X, Y, or Z.".to_string(),
                ))
            }
        }
    }
}
