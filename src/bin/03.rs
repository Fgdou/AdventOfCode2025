
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let res = input.iter()
        .map(|list| find_n_biggest(list, 2))
        .map(|list| {dbg!(&list); list})
        .map(|list| list.into_iter().fold(0, |acc, item| item as u64 + acc*10))
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let res = input.iter()
        .map(|list| find_n_biggest(list, 12))
        .map(|list| {dbg!(&list); list})
        .map(|list| list.into_iter().fold(0, |acc, item| item as u64 + acc*10))
        .sum();
    Some(res)
}

type Bank = Vec<u32>;
type Input = Vec<Bank>;

fn parse(input: &str) -> Input {
    input.split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

fn find_n_biggest(numbers: &Vec<u32>, n: usize) -> Vec<u32> {

    if n > numbers.len() {
        return vec!();
    }

    let mut list = Vec::new();
    let mut start_index = 0;

    for i in 1..=n {
        dbg!(&i, &start_index);
        let max = numbers[start_index..numbers.len()+i-n].iter().enumerate().rev().max_by_key(|item| item.1);

        match max {
            None => return vec!(),
            Some(item) => {
                list.push(*item.1);
                start_index = start_index + item.0 + 1;
            }
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_parse() {
        assert_eq!(vec!(vec!(1, 2, 3), vec!(4, 5, 6)), parse(&"123\n456\n"));
    }

    #[test]
    fn test_find_n_biggest() {
        assert_eq!(Vec::<u32>::new(), find_n_biggest(&vec!(), 1));
        assert_eq!(vec!(3), find_n_biggest(&vec!(3), 1));
        assert_eq!(vec!(3), find_n_biggest(&vec!(1, 3), 1));
        assert_eq!(Vec::<u32>::new(), find_n_biggest(&vec!(1), 2));
        assert_eq!(vec!(1, 2), find_n_biggest(&vec!(1, 2), 2));
        assert_eq!(vec!(2, 3), find_n_biggest(&vec!(1, 2, 3), 2));
        assert_eq!(vec!(9, 3), find_n_biggest(&vec!(8, 9, 3), 2));
        assert_eq!(vec!(9, 9), find_n_biggest(&vec!(8, 9, 9), 2));
        assert_eq!(vec!(9, 1), find_n_biggest(&vec!(8, 9, 1), 2));
        assert_eq!(vec!(8, 9), find_n_biggest(&vec!(8, 9), 2));
        assert_eq!(vec!(1, 1), find_n_biggest(&vec!(1, 1), 2));
        assert_eq!(vec!(7, 6), find_n_biggest(&vec!(1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3), 2));
        assert_eq!(vec!(7, 9), find_n_biggest(&vec!(1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 9), 2));
        assert_eq!(vec!(9, 9), find_n_biggest(&vec!(9, 9, 3, 4, 5, 8, 7, 6, 5, 4, 1), 2));
    }
}

// 17323
