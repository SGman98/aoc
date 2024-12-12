pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut input = input.split("\n\n");

    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.trim()
                .split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let updates = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let res = updates
        .iter()
        .filter(|update| {
            rules.iter().all(|rule| {
                let a = update.iter().position(|&x| x == rule[0]);
                let b = update.iter().position(|&x| x == rule[1]);

                if let (Some(a), Some(b)) = (a, b) {
                    a < b
                } else {
                    true
                }
            })
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>();

    Ok(res)
}

pub fn part2(_input: &str) -> Result<i32, &'static str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"47|53
                       97|13
                       97|61
                       97|47
                       75|29
                       61|13
                       75|53
                       29|13
                       97|29
                       53|29
                       61|53
                       97|53
                       61|29
                       47|13
                       75|47
                       97|75
                       47|61
                       75|61
                       47|29
                       75|13
                       53|13

                       75,47,61,53,29
                       97,61,53,29,13
                       75,29,13
                       75,97,47,61,53
                       61,13,29
                       97,13,75,29,47"#;

        let result = part1(input).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        unimplemented!()
    }
}
