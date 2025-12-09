use std::{collections::{HashMap, HashSet}, fmt::Display};

use vecmath::Vector2;

advent_of_code::solution!(9);

struct Map {
    map: Vec<Vec<char>>
}

impl Map {
    pub fn new(input: &Vec<Vector2<usize>>) -> Self {
        let x_max = input.iter().map(|i| i[0]).max().unwrap()+1;
        let y_max = input.iter().map(|i| i[1]).max().unwrap()+1;

        let mut map: Vec<Vec<char>> = (0..y_max).into_iter().map(|_| (0..x_max).into_iter().map(|_| '.').collect()).collect();

        input.iter().for_each(|p| {
            *map.get_mut(p[1]).unwrap().get_mut(p[0]).unwrap() = 'X';
        });

        Self{map}
    }

    pub fn fill_walls(&mut self, pos: &Vec<Vector2<usize>>) {
        for i in 0..pos.len() {
            let p1 = pos[i];
            let p2 = pos[(i+1)%pos.len()];

            let x_min = p1[0].min(p2[0]);
            let x_max = p1[0].max(p2[0]);
            let y_min = p1[1].min(p2[1]);
            let y_max = p1[1].max(p2[1]);

            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let c = self.map.get_mut(y).unwrap().get_mut(x).unwrap();

                    if *c == '.' {
                        *c = 'O';
                    }
                }
            }
        }
    }

    fn fill_inside_pos(&mut self, pos: Vector2<usize>) -> bool {
        let mut stack = vec!(pos);
        let mut visited = HashSet::new();

        while !stack.is_empty() {

            let pos = stack.pop().unwrap();
            if visited.contains(&pos) {
                continue;
            }

            let c = match self.map.get(pos[1]).map(|line| line.get(pos[0])).flatten() {
                None => return false,
                Some(c) => *c,
            };


            if c == '.' {
                visited.insert(pos);
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        stack.push([(pos[0] as i64 + x) as usize, (pos[1] as i64 + y) as usize])
                    }
                }
            } else {
                continue;
            }
        }

        if visited.is_empty() {
             return false;
        }

        for v in visited {
            *self.map.get_mut(v[1]).unwrap().get_mut(v[0]).unwrap() = 'O';
        }

        true
    }

    pub fn fill_inside(&mut self) {
        let max_y = self.map.len();
        let max_x = self.map.get(0).unwrap().len();
        for y in 0..max_y {
            for x in 0..max_x {
                if self.fill_inside_pos([x,y]) {
                    return;
                }
            }
        }
        unreachable!()
    }

    pub fn is_in_areas(&self, r: &(Vector2<usize>, Vector2<usize>, usize)) -> bool {
        let x_min = r.0[0].min(r.1[0]);
        let x_max = r.0[0].max(r.1[0]);
        let y_min = r.0[1].min(r.1[1]);
        let y_max = r.0[1].max(r.1[1]);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let c = self.map.get(y).unwrap().get(x).unwrap();

                if *c == '.' {
                    return false;
                }
            }
        }
        true
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            for c in line {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let sizes = get_all_sizes(&input);
    Some(sizes.iter().map(|r| r.2).max().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let (xs, ys) = (input.iter().map(|i| i[0]).collect(), input.iter().map(|i| i[1]).collect());
    let (map_x, map_y) = (create_number_map(xs), create_number_map(ys));

    let input_mapped = map_numbers(&map_x, &map_y, input.clone());

    let mut map = Map::new(&input_mapped);
    map.fill_walls(&input_mapped);
    map.fill_inside();

    let rectangles: Vec<_> = get_all_sizes(&input)
        .into_iter()
        .map(|r| (
            [*map_x.get(&r.0[0]).unwrap(), *map_y.get(&r.0[1]).unwrap()],
            [*map_x.get(&r.1[0]).unwrap(), *map_y.get(&r.1[1]).unwrap()],
            r.2,
        ))
        .collect();


    let max = rectangles.iter().filter(|r| map.is_in_areas(r)).max_by_key(|r| r.2).unwrap();

    // println!("{}", map);

    Some(max.2)
}

fn parse(input: &str) -> Vec<Vector2<usize>> {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numbers: Vec<usize> = line.split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            [*numbers.get(0).unwrap(), *numbers.get(1).unwrap()]
        }).collect()
}

fn get_size(v1: &Vector2<usize>, v2: &Vector2<usize>) -> usize {
    (v1[0].abs_diff(v2[0])+1) * (v1[1].abs_diff(v2[1])+1)
}

fn get_all_sizes(points: &Vec<Vector2<usize>>) -> Vec<(Vector2<usize>, Vector2<usize>, usize)> {
    points.iter()
        .enumerate()
        .map(|(i, p1)| {
            points[0..i].iter()
                .map(|p2| (*p1, *p2, get_size(p1, p2)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn create_number_map(numbers: HashSet<usize>) -> HashMap<usize, usize> {
    let mut numbers = numbers.into_iter().collect::<Vec<_>>();
    numbers.sort();
    numbers.into_iter().enumerate().map(|(i, v)| (v, i*2)).collect()
}

fn map_numbers(map_x: &HashMap<usize, usize>, map_y: &HashMap<usize, usize>, input: Vec<Vector2<usize>>) -> Vec<Vector2<usize>> {
    input.into_iter().map(|i| [*map_x.get(&i[0]).unwrap(), *map_y.get(&i[1]).unwrap()]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
