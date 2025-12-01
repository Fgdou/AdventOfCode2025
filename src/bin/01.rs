advent_of_code::solution!(1);

struct Move {
    direction: char,
    distance: i32,
}

fn parse_line(input: &str) -> Move {
    let c = input.chars().next().unwrap();
    let number = input[1..].parse().unwrap();

    Move{
        direction: c,
        distance: number
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<Move> = input.lines().map(parse_line).collect();

    let mut dial = 50;
    let mut count = 0;

    for m in input {
        dial += match m.direction {
            'L' => -m.distance,
            'R' => m.distance,
            _ => unreachable!()
        };

        dial = dial.rem_euclid(100);

        if dial == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Vec<Move> = input.lines().map(parse_line).collect();

    let mut dial = 50i32;
    let mut count = 0;

    for m in input {
        let dir = match m.direction {
            'L' => -1,
            'R' => 1,
            _ => unreachable!()
        };

        for _ in 0..m.distance {
            dial += dir;

            dial = dial.rem_euclid(100);


            if dial == 0 {
                count += 1;
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
