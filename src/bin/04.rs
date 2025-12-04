use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let poses: usize = input.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, c)| {
            if *c != '@' {
                return 0
            }
            if surrounded_by(&input, x as i32, y as i32) < 4 {
                1
            } else {
                0
            }
        }).sum::<usize>()
    }).sum();

    Some(poses)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars().collect()
        })
        .collect()
}

fn surrounded_by(map: &Input, x: i32, y: i32) -> usize {
    (y-1..=y+1).into_iter().filter_map(|yy| {
        let row = map.get(yy as usize)?;
        let res = (x-1..=x+1).into_iter().filter_map(|xx| {
            if x == xx && y == yy {
                return Some(0);
            }
            let c = row.get(xx as usize)?;

            if *c == '@' {
                Some(1)
            } else {
                Some(0)
            }
        }).sum::<usize>();
        Some(res)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_sunrronded() {
        let input = parse(&"...@\n.@.@\n@@@@");

        assert_eq!(1, surrounded_by(&input, 1, 0));
        assert_eq!(1, surrounded_by(&input, 0, 0));
        assert_eq!(6, surrounded_by(&input, 2, 1));
    }
}
