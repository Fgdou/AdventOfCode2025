use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let count = count_paths("you", "out", &input);

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

type Input = HashMap<String, HashSet<String>>;

fn parse(input: &str) -> Input {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut left_right = line.split(": ");
            let left = left_right.next().unwrap().to_string();
            let right = left_right.next().unwrap().split(" ").map(|s| s.to_string()).collect();

            (left, right)
        })
        .collect()
}

fn count_paths(start: &str, end: &str, map: &Input) -> usize {
    if start == end {
        1
    } else {
        let nexts = map.get(start).unwrap();
        nexts.iter().map(|next| count_paths(next, end, map)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
