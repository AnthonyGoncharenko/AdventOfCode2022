fn part_one(input: &str) -> i32 {
    let mut max = 0;
    for groups in input.split("\n\n") {
        let mut curr_max = 0;
        for line in groups.lines() {
            curr_max += line.trim().parse::<i32>().unwrap();
        }
        max = std::cmp::max(max, curr_max);
    }
    max
}
fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    let mut aggregate_list: Vec<i32> = Vec::new();

    for groups in input.split("\n\n") {
        let mut aggr = 0;
        for line in groups.lines() {
            aggr += line.trim().parse::<i32>().unwrap();
        }
        aggregate_list.push(aggr);
    }
    aggregate_list.sort();
    let len = aggregate_list.len();
    for idx in (len - 3..len).rev() {
        sum += aggregate_list.get(idx).unwrap();
    }
    sum
}

pub fn solution() -> Result<(), crate::AdventError> {
    let input = include_str!("./inputs/01.txt");

    println!("Part One's Answer: [{}]", part_one(input));
    println!("Part Two's Answer: [{}]", part_two(input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 24_000)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 45_000)
    }
}
