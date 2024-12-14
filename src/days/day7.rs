use itertools::{repeat_n, Itertools};

pub fn part1(input: &str) -> Result<isize, &'static str> {
    let res = input
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(": ");

            let first = parts.next().unwrap().parse::<isize>().unwrap();
            let rest = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            (first, rest)
        })
        .map(|(res, values)| calculate(res, values, vec!['+', '*']))
        .sum::<isize>();

    Ok(res)
}

pub fn part2(input: &str) -> Result<isize, &'static str> {
    let res = input
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(": ");

            let first = parts.next().unwrap().parse::<isize>().unwrap();
            let rest = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            (first, rest)
        })
        .map(|(res, values)| calculate(res, values, vec!['+', '*', '|']))
        .sum::<isize>();

    Ok(res)
}

fn calculate(res: isize, values: Vec<isize>, operations: Vec<char>) -> isize {
    repeat_n(operations.into_iter(), values.len() - 1)
        .multi_cartesian_product()
        .find(|combination| {
            combination
                .iter()
                .enumerate()
                .fold(values[0], |mut acc, (i, op)| {
                    match op {
                        '+' => acc += values[i + 1],
                        '*' => acc *= values[i + 1],
                        '|' => {
                            acc = format!("{}{}", acc, values[i + 1])
                                .parse::<isize>()
                                .unwrap()
                        }
                        _ => panic!("Invalid operation"),
                    }
                    acc
                })
                .eq(&res)
        })
        .map(|_| res)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"190: 10 19
                       3267: 81 40 27
                       83: 17 5
                       156: 15 6
                       7290: 6 8 6 15
                       161011: 16 10 13
                       192: 17 8 14
                       21037: 9 7 18 13
                       292: 11 6 16 20"#;

        let result = part1(input).unwrap();
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part2() {
        let input = r#"190: 10 19
                       3267: 81 40 27
                       83: 17 5
                       156: 15 6
                       7290: 6 8 6 15
                       161011: 16 10 13
                       192: 17 8 14
                       21037: 9 7 18 13
                       292: 11 6 16 20"#;

        let result = part2(input).unwrap();
        assert_eq!(result, 11387);
    }
}
