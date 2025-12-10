#![feature(trim_prefix_suffix)]

use std::{collections::{HashSet, LinkedList}, rc::Rc};

use regex::bytes::Regex;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let sum = input.iter().enumerate().map(|(i, machine)| {
        println!("{}/{}", i, input.len());
        calculate_fewest(machine)
    }).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

struct StackEntry {
    lights: Rc<Vec<bool>>,
    next_button: usize,
    count: usize,
}

fn calculate_fewest(machine: &Machine) -> usize {
    let mut stack: LinkedList<_> = machine.buttons.iter().enumerate().map(|(i, _)| StackEntry{
        count: 1,
        lights: Rc::from(machine.lights.iter().map(|_| false).collect::<Vec<_>>()),
        next_button: i,
    }).collect();

    loop {
        let mut item = stack.pop_front().unwrap();
        let button = machine.buttons.get(item.next_button).unwrap();
        item.lights = Rc::from(apply_button(button, &item.lights));

        if item.lights.as_ref() == &machine.lights {
            return item.count;
        }

        machine.buttons.iter().enumerate().map(|(i, _)| StackEntry {
            count: item.count+1,
            lights: item.lights.clone(),
            next_button: i
        })
        .for_each(|i| stack.push_back(i));
    }
}

type Input = Vec<Machine>;

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<HashSet<usize>>,
    joltage: HashSet<usize>,
}

fn apply_button(button: &HashSet<usize>, lights: &Vec<bool>) -> Vec<bool> {
    lights.iter().enumerate().map(|(i, l)| {
        if button.contains(&i) {
            !*l
        } else {
            *l
        }
    }).collect()
}

fn parse(input: &str) -> Input {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line: Vec<_> = line.split(" ").collect();
            let lights = line[0]
                .trim_prefix('[')
                .trim_suffix(']')
                .chars()
                .map(|c| c == '#')
                .collect();
            let buttons = line[1..line.iter().len()-1]
                .iter()
                .map(|button| {
                    button.trim_prefix('(')
                    .trim_suffix(')')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
                })
                .collect();
            let joltage = line[line.len()-1]
                .trim_prefix('{')
                .trim_suffix('}')
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            Machine {
                lights,
                buttons,
                joltage
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_apply_button() {
        assert_eq!(vec!(false, false), apply_button(&HashSet::from([]), &vec!(false, false)));
        assert_eq!(vec!(true, false), apply_button(&HashSet::from([0]), &vec!(false, false)));
        assert_eq!(vec!(false, false), apply_button(&HashSet::from([0]), &vec!(true, false)));
        assert_eq!(vec!(false, true), apply_button(&HashSet::from([0, 1]), &vec!(true, false)));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            vec!(Machine {
                lights: vec!(false, true, true, false),
                buttons: vec!(
                    HashSet::from([3]),
                    HashSet::from([1, 3]),
                    HashSet::from([2]),
                    HashSet::from([2, 3]),
                    HashSet::from([0, 2]),
                    HashSet::from([0, 1]),
                ),
                joltage: HashSet::from([3, 5, 4, 7])
            }),
            parse(&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
        )
    }
}
