fn part_one(input: &str) -> String {
    let mut split = input.split("\n\n");
    let first_half = split.next().unwrap();
    let cols = first_half
        .to_string()
        .lines()
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .count();
    let mut buckets: Vec<Vec<String>> = vec![];
    for _ in 0..cols {
        buckets.push(vec![]);
    }
    let mut first_half_iter = first_half.lines().rev();
    first_half_iter.next();
    for line in first_half_iter {
        for (idx, ch) in line.chars().enumerate() {
            if idx % 4 == 1 && !ch.is_ascii_whitespace() {
                buckets[(idx - 1) / 4].push(ch.to_string());
            }
        }
    }
    let second_half = split.next().unwrap();
    for line in second_half.lines() {
        let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let count = tokens[1].parse::<i32>().unwrap();
        let src = tokens[3].parse::<usize>().unwrap() - 1;
        let dest = tokens[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let temp = buckets[src].pop().unwrap();
            buckets[dest].push(temp);
        }
    }
    let mut ret = String::new();
    for mut bucket in buckets {
        let temp = bucket.pop().unwrap();
        ret.push_str(temp.as_str())
    }
    ret
}

fn part_two(input: &str) -> String {
    let mut split = input.split("\n\n");
    let first_half = split.next().unwrap();
    let cols = first_half
        .to_string()
        .lines()
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .count();
    let mut buckets: Vec<Vec<String>> = vec![];
    for _ in 0..cols {
        buckets.push(vec![]);
    }
    let mut first_half_iter = first_half.lines().rev();
    first_half_iter.next();
    for line in first_half_iter {
        for (idx, ch) in line.chars().enumerate() {
            if idx % 4 == 1 && !ch.is_ascii_whitespace() {
                buckets[(idx - 1) / 4].push(ch.to_string());
            }
        }
    }
    let second_half = split.next().unwrap();
    for line in second_half.lines() {
        let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let count = tokens[1].parse::<i32>().unwrap();
        let src = tokens[3].parse::<usize>().unwrap() - 1;
        let dest = tokens[5].parse::<usize>().unwrap() - 1;

        let mut temp_bucket: Vec<String> = Vec::new();
        for _ in 0..count {
            let temp = buckets[src].pop().unwrap();
            temp_bucket.push(temp);
        }
        temp_bucket.reverse();
        buckets[dest].extend_from_slice(&temp_bucket);
    }
    let mut ret = String::new();
    for mut bucket in buckets {
        let temp = bucket.pop().unwrap();
        ret.push_str(temp.as_str())
    }
    ret
}

pub fn solution() -> Result<(), crate::AdventError> {
    let input = include_str!("./inputs/05.txt");

    println!("Part One's Answer: [{}].", part_one(input));
    println!("Part Two's Answer: [{}].", part_two(input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), "CMZ")
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), "MCD")
    }
}
