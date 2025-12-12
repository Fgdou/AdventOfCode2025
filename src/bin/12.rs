use std::collections::{HashMap, HashSet};

use advent_of_code::template::commands::all;
use vecmath::{Vector2, vec2_add};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let res = input.trees.iter().filter(|tree| {
        let pixels_needed: usize = tree.indexes.iter().enumerate().map(|(i, n)| n* input.presents.get(&i).unwrap().len()).sum();
        let size = tree.size[0] * tree.size[1];

        if pixels_needed > size {
            return false;
        }

        let number_of_shapes: usize = tree.indexes.iter().sum();
        number_of_shapes <= tree.size[0]/3 * tree.size[1]/3
    }).count();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug, PartialEq)]
struct Tree {
    size: Vector2<usize>,
    indexes: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct Input {
    presents: HashMap<usize, HashSet<Vector2<usize>>>,
    trees: Vec<Tree>,
}

fn parse(input: &str) -> Input {
    let presents: Vec<_> = input.split("\n\n").collect();
    let trees = presents[presents.len()-1].split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut left_right = line.split(": ");
            let mut left = left_right.next().unwrap().split('x').map(|n| n.parse().unwrap());
            let right = left_right.next().unwrap().split(' ').map(|n| n.parse().unwrap()).collect();

            Tree {
                indexes: right,
                size: [left.next().unwrap(), left.next().unwrap()],
            }
        })
        .collect();
    let presents = presents[0..presents.len()-1].iter().map(|present| {
        let mut left_right = present.split('\n');
        let n = left_right.next().unwrap().trim_end_matches(":").parse().unwrap();
        let grid = left_right.enumerate().map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, c)| {
                if c != '#' {
                    return None;
                }
                Some([x, y])
            }).collect::<Vec<_>>()
        }).flatten().collect();
        (n, grid)
    }).collect();

    Input {
        presents,
        trees,
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse() {
        let input = Input {
            presents: HashMap::from([
                (0, HashSet::from([
                    [0, 0],
                    [1, 0],
                    [2, 0],
                    [0, 1],
                    [1, 1],
                    [1, 2],
                    [2, 2],
                ]))
            ]),
            trees: vec!(
                Tree {
                    size: [39, 38],
                    indexes: vec!(42, 38, 49, 36, 29, 33)
                }
            )
        };
        let str = r#"0:
###
##.
.##

39x38: 42 38 49 36 29 33"#;

        assert_eq!(input, parse(&str));
    }

    #[test]
    fn test_rotate() {
        let from = HashSet::from([
            [0, 0],
            [1, 0],
            [2, 0],
            [0, 1],
            [1, 1],
            [1, 2],
            [2, 2],
        ]);
        let to = HashSet::from([
            [2, 0],
            [2, 1],
            [2, 2],
            [1, 0],
            [1, 1],
            [0, 1],
            [0, 2],
        ]);

        assert_eq!(to, rotate(from));
    }

    #[test]
    fn test_flip_x() {
        let from = HashSet::from([
            [0, 0],
            [1, 0],
            [2, 0],
            [0, 1],
            [1, 1],
            [1, 2],
            [2, 2],
        ]);
        let to = HashSet::from([
            [0, 2],
            [1, 2],
            [2, 2],
            [0, 1],
            [1, 1],
            [1, 0],
            [2, 0],
        ]);

        assert_eq!(to, flip_x(from));
    }

    #[test]
    fn test_does_fit() {
        assert_eq!(true, does_fit_pos(&HashSet::from([]), HashSet::from([[0, 0]]), &[3, 3], &[0, 0]).is_some());
        assert_eq!(false, does_fit_pos(&HashSet::from([]), HashSet::from([[0, 0]]), &[3, 3], &[1, 1]).is_some());
        assert_eq!(false, does_fit_pos(&HashSet::from([[0, 0], [0, 2], [2, 0], [2, 2]]), HashSet::from([[0, 0]]), &[3, 3], &[0, 0]).is_some());
        assert_eq!(false, does_fit_pos(&HashSet::from([[1, 1]]), HashSet::from([[1, 1]]), &[3, 3], &[0, 0]).is_some());
        assert_eq!(true, does_fit_pos(&HashSet::from([[0, 0]]), HashSet::from([[1, 1]]), &[4, 4], &[1, 1]).is_some());
    }

    #[test]
    fn test_iter() {
        assert_eq!(vec!(0, 3, 6, 1, 2, 4, 5, 7), Size::new(8).collect::<Vec<_>>());
    }
}
