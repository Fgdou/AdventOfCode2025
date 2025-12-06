advent_of_code::solution!(6);

#[derive(Debug)]
struct Input {
    numbers: Vec<Vec<usize>>,
    operations: Vec<char>,
}

type Input2 = Vec<Vec<char>>;

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

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse2(input);
    let table = get_table(&input);

    let res = table.iter().map(|operation| calculate(operation)).sum();

    Some(res)
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

fn get_table(input: &Input2) -> MathTable {
    let operation_line: Vec<char> = input[input.len()-1].iter().cloned().collect();
    let numbers: Vec<Vec<char>> = input[0..input.len()-1].iter().cloned().collect();

    let operations: Vec<_> = operation_line.iter()
        .enumerate()
        .filter(|(_, c)| **c != ' ')
        .collect();

    let mut table = vec!();

    for i in 0..operations.len() {
        let left_index = operations.get(i).unwrap().0;
        let right_index = match operations.get(i+1) {
            None => operation_line.len(),
            Some((i, _)) => *i,
        };

        let c = *operations.get(i).unwrap().1;

        let numbers: Vec<usize> = (left_index..right_index).into_iter().filter_map(|x| {
            let numbers_str: String = (0..numbers.len()).into_iter().map(|y| {
                *numbers.get(y).unwrap().get(x).unwrap()
            }).collect();
            if numbers_str.trim().is_empty() {
                None
            } else {
                Some(numbers_str.trim().parse().unwrap())
            }
        }).collect();

        table.push(Operation { operation: c, numbers });

    }

    table
}

fn parse2(input: &str) -> Input2 {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
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
        assert_eq!(result, Some(3263827));
    }
}
