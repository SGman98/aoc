pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse::<i32>().unwrap());
        right_list.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let distance = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(distance)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse::<i32>().unwrap());
        right_list.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    let similarity = left_list
        .iter()
        .map(|left_item| {
            let right_count = right_list.iter().filter(|x| *x == left_item).count();
            right_count as i32 * left_item
        })
        .sum();

    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3   4
                       4   3
                       2   5
                       1   3
                       3   9
                       3   3"#;
        let result = part1(input).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let input = r#"3   4
                       4   3
                       2   5
                       1   3
                       3   9
                       3   3"#;
        let result = part2(input).unwrap();
        assert_eq!(result, 31);
    }
}
