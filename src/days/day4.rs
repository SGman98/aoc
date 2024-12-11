pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut count = 0;

    let mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let directions = [
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
        (1, 1),   // down-right
    ];

    let word = ['X', 'M', 'A', 'S'];

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            for (di, dj) in &directions {
                if (0..word.len()).all(|k| {
                    let ni = i as i32 + di * k as i32;
                    let nj = j as i32 + dj * k as i32;
                    ni >= 0
                        && ni < mat.len() as i32
                        && nj >= 0
                        && nj < mat[i].len() as i32
                        && mat[ni as usize][nj as usize] == word[k]
                }) {
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
            if !(mat[i][j] == 'A' && i >= 1 && i < mat.len() - 1 && j >= 1 && j < mat.len() - 1) {
                continue;
            }
            if let (('M', 'S') | ('S', 'M'), ('M', 'S') | ('S', 'M')) = (
                (mat[i - 1][j - 1], mat[i + 1][j + 1]),
                (mat[i - 1][j + 1], mat[i + 1][j - 1]),
            ) {
                count += 1
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
