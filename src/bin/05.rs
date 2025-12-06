use std::ops::RangeInclusive;

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

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn in_range(ingredient: usize, range_list: &Vec<RangeInclusive<usize>>) -> bool {
    range_list.iter().any(|range| range.contains(&ingredient))
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
        assert_eq!(result, None);
    }
}
