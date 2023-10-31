use std::collections::HashSet;

fn part_one(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut hashset: HashSet<char> = HashSet::new();
        let len = line.len();
        let chars = line.chars();
        for (idx, ch) in chars.enumerate() {
            if idx < len / 2 {
                hashset.insert(ch);
            } else if hashset.contains(&ch) {
                if ch.is_lowercase() {
                    sum += (ch as i32) - 96;
                } else {
                    sum += (ch as i32) - 64 + 26;
                }
                break;
            }
        }
    }
    sum
}
fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    let mut hashset1: HashSet<char> = HashSet::new();
    let mut hashset2: HashSet<char> = HashSet::new();

    for (idx, line) in input.lines().enumerate() {
        match idx % 3 {
            0 => {
                hashset1.clear();
                hashset2.clear();
                for ch in line.chars() {
                    hashset1.insert(ch);
                }
            }
            1 => {
                for ch in line.chars() {
                    if hashset1.contains(&ch) {
                        hashset2.insert(ch);
                    }
                }
            }
            2 => {
                for ch in line.chars() {
                    if hashset2.contains(&ch) {
                        if ch.is_lowercase() {
                            sum += (ch as i32) - 96;
                        } else {
                            sum += (ch as i32) - 64 + 26;
                        }
                        break;
                    }
                }
            }
            _ => unreachable!("Mod 3 => 0..=2"),
        }
    }
    sum
}

pub fn solution() -> Result<(), crate::AdventError> {
    let input = include_str!("./inputs/03.txt");

    println!("Part One's Answer is : [{}].", part_one(input));
    println!("Part Two's Answer is : [{}].", part_two(input));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 157)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 70)
    }
}
