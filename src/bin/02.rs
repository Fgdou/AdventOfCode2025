#![feature(new_range_api)]
use std::range::{Range, RangeInclusive};

advent_of_code::solution!(2);

type Input = Vec<RangeInclusive<u64>>;

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);


    let sum = input.iter().map(|range|
        range.iter()
            .filter_map(|n|
                if check_double(n) {
                    Some(n as u64)
                } else {
                    None
                }
            )
            .sum::<u64>()
    )
    .sum();

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn check_double(n: u64) -> bool {
    let str_representation = n.to_string();
    let length = str_representation.len();

    let (left, right) = str_representation.split_at(length/2);

    return left == right;
}

fn parse_line(line: &str) -> RangeInclusive<u64> {
    let numbers: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
    assert!(numbers.len() == 2);
    RangeInclusive::from(numbers[0]..=numbers[1])
}

fn parse(input: &str) -> Input {
    input.split('\n')
        .next()
        .unwrap()
        .split(',')
        .map(parse_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_double() {
        assert_eq!(true, check_double(11));
        assert_eq!(false, check_double(101));
        assert_eq!(true, check_double(5555));
        assert_eq!(true, check_double(6565));
        assert_eq!(true, check_double(123123));
        assert_eq!(false, check_double(1230123));
        assert_eq!(true, check_double(1188511885))
    }
}
