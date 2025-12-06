advent_of_code::solution!(6);

#[derive(Debug)]
struct Input {
    numbers: Vec<Vec<usize>>,
    operations: Vec<char>,
}

type MathTable = Vec<Operation>;

#[derive(Debug)]
struct Operation {
    operation: char,
    numbers: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let table = transform_to_table(&input);

    let res = table.iter().map(|operation| calculate(operation)).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn transform_to_table(input: &Input) -> MathTable {
    let width = input.numbers.get(0).unwrap().len();
    let height = input.numbers.len();

    (0..width).into_iter().map(|x| {
        let numbers = (0..height).into_iter().map(|y| {
            *input.numbers.get(y).unwrap().get(x).unwrap()
        }).collect();

        let operation = *input.operations.get(x).unwrap();

        Operation {
            numbers,
            operation,
        }
    }).collect()
}

fn calculate(operation: &Operation) -> usize {
    let first = match operation.operation {
        '+' => 0,
        '*' => 1,
        _ => unreachable!()
    };
    operation.numbers.iter().fold(first, |acc, n| {
        match operation.operation {
            '+' => acc + n,
            '*' => acc * n,
            _ => unreachable!()
        }
    })
}

fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.split('\n').filter(|line| !line.is_empty()).collect();

    let numbers = lines[0..lines.len()-1].iter().map(|line| {
        line.split_whitespace().filter(|c| !c.is_empty()).map(|n| n.parse().unwrap()).collect()
    }).collect();
    let operations = lines[lines.len()-1].split_whitespace().map(|c| c.chars().next().unwrap()).collect();

    Input{
        operations,
        numbers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
