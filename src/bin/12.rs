use std::collections::{HashMap, HashSet};

use advent_of_code::template::commands::all;
use vecmath::{Vector2, vec2_add};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let res = input.trees.iter().enumerate().filter(|(i, tree)| {
        let res = try_fit(&mut HashSet::new(), tree.indexes.clone(), &input.presents, &tree.size, [0,0]);
        println!("{} {}", i, res);
        res
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

fn rotate(tree: HashSet<Vector2<usize>>) -> HashSet<Vector2<usize>> {
    tree.into_iter().map(|p| {
        match p {
            [0, 0] => [2, 0],
            [1, 0] => [2, 1],
            [2, 0] => [2, 2],
            [2, 1] => [1, 2],
            [2, 2] => [0, 2],
            [1, 2] => [0, 1],
            [0, 2] => [0, 0],
            [0, 1] => [1, 0],
            _ => p
        }
    }).collect()
}

fn flip_x(tree: HashSet<Vector2<usize>>) -> HashSet<Vector2<usize>> {
    tree.into_iter().map(|p| [p[0], flip_n(p[1])]).collect()
}

fn flip_y(tree: HashSet<Vector2<usize>>) -> HashSet<Vector2<usize>> {
    tree.into_iter().map(|p| [flip_n(p[0]), p[1]]).collect()
}

fn flip_n(n: usize) -> usize {
    match n {
        0 => 2,
        2 => 0,
        _ => n,
    }
}

fn does_fit_pos(grid: &HashSet<Vector2<usize>>, mut present: HashSet<Vector2<usize>>, size: &Vector2<usize>, start_pos: &Vector2<usize>) -> Option<HashSet<Vector2<usize>>> {
    if start_pos[0]+3 > size[0] || start_pos[1]+3 > size[1] {
        return None;
    }

    for i in 0..4 {
        if i > 0 {
            present = rotate(present);
        }

        let tests = vec!(
            present.clone(),
            flip_x(present.clone()),
            flip_y(present.clone()),
        );

        for present in tests {
            let all_fit = present.iter().all(|pos| {
                let pos = vec2_add(pos.clone(), start_pos.clone());
                !grid.contains(&pos)
            });
            if all_fit {
                return Some(present);
            }
        }
    }
    None
}

struct Size {
    size: usize,
    state: usize
}

impl Size {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            state: 0,
        }
    }
}

impl Iterator for Size {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state;
        if self.state % 3 == 0 {
            self.state += 3;

            if self.state >= self.size {
                self.state = 1;
            }

            Some(state)
        } else {
            if self.state % 3 == 1 {
                self.state += 1;
            } else {
                self.state += 2;
            }

            if state >= self.size {
                None
            } else {
                Some(state)
            }
        }
    }
}

fn try_fit(grid: &mut HashSet<Vector2<usize>>, mut presents_left: Vec<usize>, presents: &HashMap<usize, HashSet<Vector2<usize>>>, size: &Vector2<usize>, mut start_pos: Vector2<usize>) -> bool {
    // dbg!(&presents_left, start_pos);
    // print_grid(grid, size);
    if presents_left.iter().all(|p| *p == 0) {
        return true;
    }

    for (i, n) in presents_left.clone().iter().enumerate() {
        if *n == 0 {
            continue;
        }

        let present = presents.get(&i).unwrap();
        loop {
            if let Some(present) = does_fit_pos(grid, present.clone(), size, &start_pos) {
                *presents_left.get_mut(i).unwrap() -= 1;
                present.iter().for_each(|p| {grid.insert(vec2_add(*p,start_pos));});

                let mut start_pos2 = start_pos.clone();
                if start_pos2[1] > 0 {
                    start_pos2[1] -= 1;
                }
                start_pos2[0] += 1;

                let res = try_fit(grid, presents_left.clone(), presents, size, start_pos2);
                *presents_left.get_mut(i).unwrap() += 1;
                present.iter().for_each(|p| {grid.remove(&vec2_add(*p,start_pos));});

                return res;
            }

            start_pos[0] += 1;
            if start_pos[0] >= size[0] {
                start_pos[0] = 0;
                start_pos[1] += 1;
            }
            if start_pos[1] >= size[1] {
                return false;
            }
        }
    }
    false
}

fn print_grid(grid: &HashSet<Vector2<usize>>, size: &Vector2<usize>) {
    for y in 0..size[1] {
        for x in 0..size[0] {
            if grid.contains(&[x, y]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
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
