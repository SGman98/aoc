#[derive(Debug)]
enum Direction {
    None,
    Asc,
    Desc,
}

pub fn check_safe(levels: Vec<i32>) -> Result<(), ()> {
    levels
        .windows(2)
        .map(|w| w[0] - w[1])
        .try_fold(Direction::None, |acc, x| match (acc, x) {
            (Direction::Asc | Direction::None, 1..=3) => Ok(Direction::Asc),
            (Direction::Desc | Direction::None, -3..=-1) => Ok(Direction::Desc),
            _ => Err(()),
        })
        .map(|_| ())
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut safe = 0;
    for line in input.lines() {
        let levels = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let res = check_safe(levels);
        if res.is_ok() {
            safe += 1;
        }
    }
    Ok(safe)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
        for i in 0..levels.len() {
            let mut levels_cloned = levels.clone();
            levels_cloned.remove(i);
            let res = check_safe(levels_cloned);
            if res.is_ok() {
                safe += 1;
                break;
            }
        }
    }
    Ok(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"7 6 4 2 1
                       1 2 7 8 9
                       9 7 6 2 1
                       1 3 2 4 5
                       8 6 4 4 1
                       1 3 6 7 9"#;
        let result = part1(input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let input = r#"7 6 4 2 1
                       1 2 7 8 9
                       9 7 6 2 1
                       1 3 2 4 5
                       8 6 4 4 1
                       1 3 6 7 9"#;
        let result = part2(input).unwrap();
        assert_eq!(result, 4);
    }
}
