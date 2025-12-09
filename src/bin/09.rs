use vecmath::Vector2;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let sizes = get_all_sizes(&input);
    Some(*sizes.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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

fn get_all_sizes(points: &Vec<Vector2<usize>>) -> Vec<usize> {
    points.iter()
        .enumerate()
        .map(|(i, p1)| {
            points[0..i].iter()
                .map(|p2| get_size(p1, p2))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
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
        assert_eq!(result, None);
    }
}
