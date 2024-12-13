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
        .map(|(res, values)| {
            let len = values.len();
            let max_range = (1 << len) - 1;
            let mut possible = false;
            'op: for i in 0..max_range {
                let mut sum = values[0];
                for (j, value) in values.iter().enumerate().skip(1) {
                    if (i >> (j - 1)) & 1 == 0 {
                        sum *= value;
                    } else {
                        sum += value;
                    }
                }

                if sum == res {
                    possible = true;
                    break 'op;
                }
            }
            if possible {
                res
            } else {
                0
            }
        })
        .sum::<isize>();

    Ok(res)
}

pub fn part2(_input: &str) -> Result<isize, &'static str> {
    unimplemented!()
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
        unimplemented!()
    }
}
