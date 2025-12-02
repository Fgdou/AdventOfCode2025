#![feature(new_range_api)]
use std::{collections::HashSet, range::RangeInclusive};

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
    let input = parse(input);


    let sum = input.iter().map(|range|
        range.iter()
            .filter_map(|n|
                if check_double_or_more(n) {
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

fn all_element_same(list: Vec<&str>) -> bool {
    let set: HashSet<&str> = HashSet::from_iter(list.iter().cloned());

    set.len() == 1
}

fn check_double_or_more(n: u64) -> bool {
    let txt = n.to_string();

    for i in 2..=txt.len() {
        let list = split_by(&txt, i as usize);

        let all_same = match list {
            None => continue,
            Some(vec) => all_element_same(vec)
        };

        if all_same {
            return true
        }
    };

    false
}

fn split_by(txt: &str, n: usize) -> Option<Vec<&str>> {
    if txt.len() % n != 0 {
        return None;
    }

    let delta = txt.len() / n;

    let res = (0..n).into_iter()
        .map(|i| &txt[delta*i..delta*(i+1)])
        .collect();

    Some(res)
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
        assert_eq!(result, Some(4174379265));
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

    #[test]
    fn test_split_by() {
        assert_eq!(Some(vec!("")), split_by("", 1));
        assert_eq!(Some(vec!("123", "123")), split_by("123123", 2));
        assert_eq!(Some(vec!("123", "123", "123")), split_by("123123123", 3));
        assert_eq!(Some(vec!("12", "31", "23")), split_by("123123", 3));
        assert_eq!(None, split_by("123123", 4));
    }

    #[test]
    fn test_more_than_double() {
        assert_eq!(false, check_double_or_more(123));
        assert_eq!(true, check_double_or_more(1212));
        assert_eq!(true, check_double_or_more(11));
        assert_eq!(true, check_double_or_more(1111));
        assert_eq!(true, check_double_or_more(123123123));
    }
}
