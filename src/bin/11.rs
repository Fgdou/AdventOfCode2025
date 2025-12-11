use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let count = count_paths("you", "out", &input);

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let conditions = ["dac".to_string(), "fft".to_string()];

    let count = count_paths_pass_through("svr", "out", &input, &mut HashSet::from(conditions), &mut HashMap::new());

    Some(count)
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

fn count_paths_pass_through(start: &str, end: &str, map: &Input, conditions: &mut HashSet<String>, visited: &mut HashMap<(String, Vec<String>), usize>) -> usize {
    let key = (start.to_string(), conditions.iter().cloned().collect());

    if let Some(n) = visited.get(&key) {
        return *n;
    }

    let removed = conditions.remove(start);
    let result = if start == end {
        match conditions.is_empty() {
            true => 1,
            false => 0,
        }
    } else {
        let nexts = map.get(start).unwrap();
        nexts.iter().map(|next| count_paths_pass_through(next, end, map, conditions, visited)).sum()
    };
    if removed {
        conditions.insert(start.to_string());
    }
    visited.insert(key, result);
    result
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
