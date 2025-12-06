#![feature(get_disjoint_mut_helpers)]

use core::slice::GetDisjointMutIndex;
use std::{collections::HashSet, ops::RangeInclusive};

advent_of_code::solution!(5);

struct Input {
    range_list: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let ingredients: Vec<_> = input.ingredients.iter()
        .filter(|i| in_range(**i, &input.range_list))
        .collect();

    Some(ingredients.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let mut ranges = input.range_list;
    ranges.sort_by_key(|obj| *obj.start());

    let mut simplified: Vec<RangeInclusive<usize>> = vec!();

    for range in ranges {
        if simplified.is_empty() {
            simplified.push(range.clone());
            continue;
        }
        let first = simplified.remove(simplified.len()-1);
        simplify(&first, &range).into_iter()
            .for_each(|i| simplified.push(i));
    }

    let res = simplified.iter().map(|range| range.end() - range.start() + 1).sum();

    Some(res)
}

fn in_range(ingredient: usize, range_list: &Vec<RangeInclusive<usize>>) -> bool {
    range_list.iter().any(|range| range.contains(&ingredient))
}

fn simplify(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> Vec<RangeInclusive<usize>> {
    if first.is_overlapping(second) {
        vec!(usize::min(*first.start(), *second.start())..=usize::max(*first.end(), *second.end()))
    } else {
        vec!(first.clone(), second.clone())
    }
}

fn parse(input: &str) -> Input {
    let mut splitted = input.split("\n\n");
    let range_list: Vec<_> = splitted.next().unwrap()
        .split("\n")
        .map(|line| {
            let mut splitted = line.split('-');
            let left: usize = splitted.next().unwrap().parse().unwrap();
            let right: usize = splitted.next().unwrap().parse().unwrap();
            left..=right
        })
        .collect();
    let ingredients: Vec<usize> = splitted.next().unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    Input { range_list, ingredients }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_simplify() {
        assert_eq!(vec!(0..=1, 2..=3), simplify(&(0..=1), &(2..=3)));
        assert_eq!(vec!(0..=3), simplify(&(0..=2), &(2..=3)));
        assert_eq!(vec!(0..=9), simplify(&(0..=9), &(2..=3)));
    }
}
