pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut count = 0;

    let mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == 'X' {
                // up
                if i >= 3 && mat[i - 1][j] == 'M' && mat[i - 2][j] == 'A' && mat[i - 3][j] == 'S' {
                    count += 1;
                }
                // down
                if i < mat.len() - 3
                    && mat[i + 1][j] == 'M'
                    && mat[i + 2][j] == 'A'
                    && mat[i + 3][j] == 'S'
                {
                    count += 1;
                }
                // left
                if j >= 3 && mat[i][j - 1] == 'M' && mat[i][j - 2] == 'A' && mat[i][j - 3] == 'S' {
                    count += 1;
                }
                // right
                if j < mat[i].len() - 3
                    && mat[i][j + 1] == 'M'
                    && mat[i][j + 2] == 'A'
                    && mat[i][j + 3] == 'S'
                {
                    count += 1;
                }
                // up-left
                if i >= 3
                    && j >= 3
                    && mat[i - 1][j - 1] == 'M'
                    && mat[i - 2][j - 2] == 'A'
                    && mat[i - 3][j - 3] == 'S'
                {
                    count += 1;
                }
                // up-right
                if i >= 3
                    && j < mat[i].len() - 3
                    && mat[i - 1][j + 1] == 'M'
                    && mat[i - 2][j + 2] == 'A'
                    && mat[i - 3][j + 3] == 'S'
                {
                    count += 1;
                }
                // down-left
                if i < mat.len() - 3
                    && j >= 3
                    && mat[i + 1][j - 1] == 'M'
                    && mat[i + 2][j - 2] == 'A'
                    && mat[i + 3][j - 3] == 'S'
                {
                    count += 1;
                }
                // down-right
                if i < mat.len() - 3
                    && j < mat[i].len() - 3
                    && mat[i + 1][j + 1] == 'M'
                    && mat[i + 2][j + 2] == 'A'
                    && mat[i + 3][j + 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let mut count = 0;

    let mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == 'A' && i >= 1 && i < mat.len() - 1 && j >= 1 && j < mat.len() - 1 {
                match (
                    mat[i - 1][j - 1],
                    mat[i + 1][j + 1],
                    mat[i - 1][j + 1],
                    mat[i + 1][j - 1],
                ) {
                    ('M', 'S', 'M', 'S')
                    | ('M', 'S', 'S', 'M')
                    | ('S', 'M', 'M', 'S')
                    | ('S', 'M', 'S', 'M') => count += 1,
                    _ => (),
                }
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"MMMSXXMASM
                       MSAMXMSMSA
                       AMXSXMAAMM
                       MSAMASMSMX
                       XMASAMXAMM
                       XXAMMXXAMA
                       SMSMSASXSS
                       SAXAMASAAA
                       MAMMMXMMMM
                       MXMXAXMASX"#;

        let result = part1(input).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let input = r#"MMMSXXMASM
                       MSAMXMSMSA
                       AMXSXMAAMM
                       MSAMASMSMX
                       XMASAMXAMM
                       XXAMMXXAMA
                       SMSMSASXSS
                       SAXAMASAAA
                       MAMMMXMMMM
                       MXMXAXMASX"#;

        let result = part2(input).unwrap();
        assert_eq!(result, 9);
    }
}
