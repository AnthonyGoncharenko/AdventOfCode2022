mod rps_strategy;
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
fn main() {
    let input = include_str!("./input.txt");
    let ans1 = part_one(&input);
    println!("Part One's Score: [{ans1}]");
    let ans2 = part_two(&input);
    println!("Part Two's Score: [{ans2}]");
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
