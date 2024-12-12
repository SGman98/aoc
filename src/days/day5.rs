use std::collections::{HashMap, HashSet};

fn check_next_pages(
    last_entry: (i32, HashSet<i32>),
    pages_hash: &mut HashMap<i32, (i32, HashSet<i32>)>,
) {
    let last_index = last_entry.0;
    for page in last_entry.1 {
        let next_entry = pages_hash
            .entry(page)
            .or_insert((last_index, HashSet::new()));
        if next_entry.0 > last_index {
            continue;
        }
        next_entry.0 = last_index + 1;
        if next_entry.1.is_empty() {
            continue;
        }
        check_next_pages(next_entry.clone(), pages_hash);
    }
}

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

pub fn part2(input: &str) -> Result<i32, &'static str> {
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
            !rules.iter().all(|rule| {
                let a = update.iter().position(|&x| x == rule[0]);
                let b = update.iter().position(|&x| x == rule[1]);

                if let (Some(a), Some(b)) = (a, b) {
                    a < b
                } else {
                    true
                }
            })
        })
        .map(|line| {
            let mut tmp = line.clone();
            let mut pages_hash: HashMap<i32, (i32, HashSet<i32>)> = HashMap::new();
            for rule in &rules {
                if !line.contains(&rule[0]) || !line.contains(&rule[1]) {
                    continue;
                }
                let first = pages_hash.entry(rule[0]).or_insert((0, HashSet::new()));
                first.1.insert(rule[1]);
                check_next_pages(first.clone(), &mut pages_hash);
            }

            let pages_order = pages_hash
                .iter()
                .map(|item| (*item.0, item.1 .0))
                .collect::<Vec<(i32, i32)>>();

            tmp.sort_by(|a, b| {
                let a = pages_order.iter().find(|item| item.0 == *a).unwrap().1;
                let b = pages_order.iter().find(|item| item.0 == *b).unwrap().1;
                a.cmp(&b)
            });
            tmp
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>();

    Ok(res)
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

        let result = part2(input).unwrap();
        assert_eq!(result, 123);
    }
}
