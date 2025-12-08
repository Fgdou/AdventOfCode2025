
use std::collections::{HashMap, HashSet};

use vecmath::Vector3;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Graph {
    graphs: HashMap<Vector3<usize>, HashSet<Vector3<usize>>>
}

impl Graph {
    pub fn connect(&mut self, v1: Vector3<usize>, v2: Vector3<usize>) -> bool {
        if self.are_connected(&v1, &v2) {
            return false;
        }
        if let Some(v) = self.graphs.get_mut(&v1) {
            v.insert(v2.clone());
        } else {
            self.graphs.insert(v1.clone(), HashSet::from([v2.clone()]));
        }
        if let Some(v) = self.graphs.get_mut(&v2) {
            v.insert(v1);
        } else {
            self.graphs.insert(v2, HashSet::from([v1]));
        }
        true
    }

    pub fn are_connected(&self, v1: &Vector3<usize>, v2: &Vector3<usize>) -> bool {
        let mut visited = HashSet::new();
        self.visit(v1.clone(), &mut visited).contains(v2)
    }

    pub fn new(vecs: &HashSet<Vector3<usize>>) -> Self {
        let graphs = vecs.iter().map(|v| (v.clone(), HashSet::new())).collect();
        Self{graphs}
    }

    pub fn get_graphs(&self) -> Vec<HashSet<Vector3<usize>>> {
        let mut visited = HashSet::new();

        self.graphs.keys().into_iter().filter_map(|pos| {
            if visited.contains(pos) {
                None
            } else {
                Some(self.visit(pos.clone(), &mut visited))
            }
        }).collect()
    }

    pub fn get_first_graph_size(&self) -> usize {
        let pos = self.graphs.keys().next().unwrap();
        let mut visited = HashSet::new();
        self.visit(*pos, &mut visited).len()
    }

    fn visit(&self, pos: Vector3<usize>, visited: &mut HashSet<Vector3<usize>>) -> HashSet<Vector3<usize>> {
        if visited.contains(&pos) {
            return HashSet::new();
        }
        visited.insert(pos.clone());

        let mut map = match self.graphs.get(&pos) {
            None => HashSet::new(),
            Some(others) => others.iter().map(|other| {
                self.visit(*other, visited)
            }).flatten().collect()
        };
        map.insert(pos);

        map
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_n(input, 1000)
}

fn part_one_n(input: &str, n: usize) -> Option<usize> {
    let input = parse(input);
    let mut distances = calculate_distance(&input);
    let mut graph = Graph::new(&input);

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    distances.reverse();

    for _ in 0..n {
        let next = distances.pop().unwrap();

        graph.connect(next.0, next.1);
    }

    let mut graphs = graph.get_graphs();
    graphs.sort_by_key(|g| g.len());
    graphs.reverse();

    let res = calculate_product(&graphs);

    Some(res)
}

fn calculate_product(graphs: &Vec<HashSet<Vector3<usize>>>) -> usize {
    graphs[0..3].iter().map(|g| g.len()).product()
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut distances = calculate_distance(&input);
    let mut graph = Graph::new(&input);

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    distances.reverse();

    let mut history = None;

    let mut graph_size = 0;

    loop {
        if graph_size == input.len() {
            break;
        }
        let next = distances.pop();

        // println!("{} {}", distances.len(), graph_size);

        match next {
            None => break,
            Some(next) => {
                if graph.connect(next.0, next.1) {
                    history = Some(next);
                    graph_size = graph.get_first_graph_size();
                }
            }
        }
    };

    history.map(|h| h.0[0] * h.1[0])
}

type Input = HashSet<Vector3<usize>>;

fn parse(input: &str) -> Input {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut numbers = line.split(',').map(|number| number.parse().unwrap());
            [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()]
        }).collect()
}

fn distance(b1: &Vector3<usize>, b2: &Vector3<usize>) -> f64 {
    let x = (b1[0] as f64 - b2[0] as f64).powi(2);
    let y = (b1[1] as f64 - b2[1] as f64).powi(2);
    let z = (b1[2] as f64 - b2[2] as f64).powi(2);

    let sum = x+y+z;

    sum.sqrt()
}

fn calculate_distance(boxes: &HashSet<Vector3<usize>>) -> Vec<(Vector3<usize>, Vector3<usize>, f64)> {
    let boxes = Vec::from_iter(boxes.iter().cloned());
    boxes.iter()
        .enumerate()
        .map(|(i, box1)| boxes[0..i].iter().map(|box2| {
            (box1.clone(), box2.clone(), distance(&box1, box2))
        }).collect::<Vec<_>>())
        .flatten()
        .filter(|(b1, b2, _)| b1 != b2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_n(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }

    #[test]
    fn test_calculate_distance() {
        assert_eq!(vec!(([1, 1, 1], [1, 1, 2], 1.0)), calculate_distance(&HashSet::from([[1, 1, 1], [1, 1, 2]])))
    }

    #[test]
    fn test_graph() {
        let mut graph = Graph::new(&HashSet::new());

        graph.connect([1, 1, 1], [1, 1, 2]);
        graph.connect([2, 2, 2], [2, 2, 3]);

        assert_eq!(vec!(
            HashSet::from([[2, 2, 2], [2, 2, 3]]),
            HashSet::from([[1, 1, 1], [1, 1, 2]]),
        ), graph.get_graphs())
    }

    #[test]
    fn test_connected() {
        let mut graph = Graph::new(&HashSet::new());

        graph.connect([1, 1, 1], [1, 1, 2]);
        graph.connect([2, 2, 2], [2, 2, 3]);

        assert_eq!(false, graph.are_connected(&[1, 1, 1], &[2, 2, 2]));
        assert_eq!(true, graph.are_connected(&[1, 1, 1], &[1, 1, 2]));
    }
}
