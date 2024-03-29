fn part_one(input: &str) -> i32 {
    let mut ret = 0;
    for line in input.lines() {
        let ranges = line.split(',').collect::<Vec<_>>();
        let elf_1 = ranges.first().unwrap().split('-').collect::<Vec<_>>();
        let elf_2 = ranges.get(1).unwrap().split('-').collect::<Vec<_>>();

        let elf_1_l = elf_1.first().unwrap().parse::<i32>().unwrap();
        let elf_1_r = elf_1.get(1).unwrap().parse::<i32>().unwrap();
        let elf_2_l = elf_2.first().unwrap().parse::<i32>().unwrap();
        let elf_2_r = elf_2.get(1).unwrap().parse::<i32>().unwrap();

        if elf_2_l <= elf_1_l && elf_1_r <= elf_2_r || elf_1_l <= elf_2_l && elf_2_r <= elf_1_r {
            ret += 1;
        }
    }
    ret
}

fn part_two(input: &str) -> i32 {
    let mut ret = 0;
    for line in input.lines() {
        let ranges = line.split(',').collect::<Vec<_>>();
        let elf_1 = ranges.first().unwrap().split('-').collect::<Vec<_>>();
        let elf_2 = ranges.get(1).unwrap().split('-').collect::<Vec<_>>();

        let elf_1_l = elf_1.first().unwrap().parse::<i32>().unwrap();
        let elf_1_r = elf_1.get(1).unwrap().parse::<i32>().unwrap();
        let elf_2_l = elf_2.first().unwrap().parse::<i32>().unwrap();
        let elf_2_r = elf_2.get(1).unwrap().parse::<i32>().unwrap();

        if elf_1_l <= elf_2_l && elf_2_l <= elf_1_r
            || elf_2_l <= elf_1_l && elf_1_l <= elf_2_r
            || elf_1_l <= elf_2_r && elf_2_r <= elf_1_r
            || elf_2_l <= elf_1_r && elf_1_r <= elf_2_r
        {
            ret += 1;
        }
    }
    ret
}

pub fn solution() -> Result<(), crate::AdventError> {
    let input = include_str!("./inputs/04.txt");

    println!("Part One's Answer: [{}].", part_one(input));
    println!("Part Two's Answer: [{}].", part_two(input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4)
    }
}
