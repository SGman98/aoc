pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut res = 0;
    for line in input.lines() {
        let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for cap in re.captures_iter(line) {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            res += a * b;
        }
    }
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
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let result = part1(input).unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        unimplemented!()
    }
}
