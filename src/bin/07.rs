use std::collections::{HashMap, HashSet};

use vecmath::Vector2;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq)]
struct Input {
    splitters: HashSet<Vector2<usize>>,
    start: Vector2<usize>,
    height: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Splitted {
    rays: Vec<Vector2<usize>>,
    splitter: Option<Vector2<usize>>
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let mut stack = vec!(input.start);
    let mut splitters = HashSet::new();
    let mut visited = HashSet::new();

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos.clone());

        if pos[1] >= input.height {
            continue;
        } else {
            let splitter = move_ray(pos, &input.splitters);
            splitter.splitter.map(|s| splitters.insert(s));
            splitter.rays.into_iter().for_each(|ray| stack.push(ray));
        }
    }

    Some(splitters.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn parse(input: &str) -> Input {
    let mut res: Input = Input{
        splitters: HashSet::new(),
        start: [0, 0],
        height: 0,
    };
    for (y, line) in input.split('\n').enumerate() {
        res.height = usize::max(res.height, y+1);
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                res.start = [x, y];
            } else if c == '^' {
                res.splitters.insert([x, y]);
            }
        }
    }
    res
}

fn move_ray(pos: Vector2<usize>, splitters: &HashSet<Vector2<usize>>) -> Splitted {
    let pos = [pos[0], pos[1]+1];

    match splitters.contains(&pos) {
        false => Splitted { rays: vec!(pos), splitter: None },
        true => {
            Splitted {
                rays: vec!(
                    [pos[0]-1, pos[1]],
                    [pos[0]+1, pos[1]],
                ),
                splitter: Some(pos)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse() {
        let str = r#"...S...
...^...
..^.^.."#;
        let res = Input {
            start: [3, 0],
            splitters: HashSet::from([[3, 1], [2, 2], [4, 2]]),
            height: 3,
        };
        assert_eq!(res, parse(&str));
    }

    #[test]
    fn test_move_ray() {
        let set = HashSet::from([[1, 2]]);

        assert_eq!(Splitted{rays: vec!([1, 1]), splitter: None}, move_ray([1, 0], &set));
        assert_eq!(Splitted{rays: vec!([0, 2], [2, 2]), splitter: Some([1, 2])}, move_ray([1, 1], &set));
    }
}
