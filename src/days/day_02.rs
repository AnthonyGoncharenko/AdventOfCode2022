use core::panic;
use std::str::FromStr;

fn part_one(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let choices = line.split_ascii_whitespace().collect::<Vec<_>>();
        let opp_choice = match rps_strategy::OpponentStrategy::from_str(choices.get(0).unwrap()) {
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

        let opp_choice = rps_strategy::RPS::from_opponent(opp_choice);
        let my_choice = rps_strategy::RPS::from_me(my_choice);
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
        let opp_choice = match rps_strategy::OpponentStrategy::from_str(choices.get(0).unwrap()) {
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

        let opp_choice = rps_strategy::RPS::from_opponent(opp_choice);
        let my_choice = rps_strategy::RPS::from_me_truth(&my_choice, &opp_choice);
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

    println!("Part One's Score: [{}]", part_one(&input));
    println!("Part Two's Score: [{}]", part_two(&input));
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
        assert_eq!(part_one(&TEST_INPUT), 15)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 12)
    }
}

mod rps_strategy {
    use std::{cmp::Ordering, str::FromStr};

    #[derive(PartialOrd, Eq, PartialEq)]
    pub enum RPS {
        ROCK,
        PAPER,
        SCISSORS,
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

    impl RPS {
        pub fn from_opponent(opp_strat: OpponentStrategy) -> RPS {
            use OpponentStrategy as OS;
            match opp_strat {
                OS::A => Self::ROCK,
                OS::B => Self::PAPER,
                OS::C => Self::SCISSORS,
            }
        }
        pub fn from_me(my_strat: MyStrategy) -> RPS {
            use MyStrategy as MS;
            match my_strat {
                MS::X => Self::ROCK,
                MS::Y => Self::PAPER,
                MS::Z => Self::SCISSORS,
            }
        }
        pub fn from_me_truth(my_strat: &MyStrategy, opp_strat: &RPS) -> RPS {
            use MyStrategy as MS;
            match opp_strat {
                // Rock
                Self::ROCK => match my_strat {
                    MS::X => Self::SCISSORS, // Lose
                    MS::Y => Self::ROCK,     // Draw
                    MS::Z => Self::PAPER,    // Win
                },
                // Paper
                Self::PAPER => match my_strat {
                    MS::X => Self::ROCK,
                    MS::Y => Self::PAPER,
                    MS::Z => Self::SCISSORS,
                },
                // Scissors
                Self::SCISSORS => match my_strat {
                    MS::X => Self::PAPER,
                    MS::Y => Self::SCISSORS,
                    MS::Z => Self::ROCK,
                },
            }
        }
    }
    impl Ord for RPS {
        fn cmp(&self, other: &Self) -> Ordering {
            match self {
                RPS::PAPER => match other {
                    RPS::PAPER => Ordering::Equal,
                    RPS::ROCK => Ordering::Greater,
                    RPS::SCISSORS => Ordering::Less,
                },
                RPS::ROCK => match other {
                    RPS::PAPER => Ordering::Less,
                    RPS::ROCK => Ordering::Equal,
                    RPS::SCISSORS => Ordering::Greater,
                },
                RPS::SCISSORS => match other {
                    RPS::PAPER => Ordering::Greater,
                    RPS::ROCK => Ordering::Less,
                    RPS::SCISSORS => Ordering::Equal,
                },
            }
        }
    }
    impl From<RPS> for i32 {
        fn from(val: RPS) -> Self {
            match val {
                RPS::ROCK => 1,
                RPS::PAPER => 2,
                RPS::SCISSORS => 3,
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
