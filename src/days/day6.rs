#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        matches!(self, Direction::North | Direction::South)
    }

    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::East | Direction::West)
    }
}

fn navigate(mat: &mut Vec<Vec<char>>, pos: (i32, i32), cur_direction: Direction) -> bool {
    let next_pos = match cur_direction {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
    };

    if next_pos.0 < 0
        || next_pos.0 >= mat.len() as i32
        || next_pos.1 < 0
        || next_pos.1 >= mat[0].len() as i32
    {
        mat[pos.0 as usize][pos.1 as usize] = 'X';
        return false;
    }

    let next_char = mat[next_pos.0 as usize][next_pos.1 as usize];

    if next_char == '#' {
        let new_direction = match cur_direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        let cur_char = mat[pos.0 as usize][pos.1 as usize];
        if (cur_direction.is_vertical() && cur_char == '|')
            || (cur_direction.is_horizontal() && cur_char == '-')
            || cur_char == '+'
        {
            return true;
        }

        if cur_char == '.' {
            if cur_direction.is_vertical() {
                mat[pos.0 as usize][pos.1 as usize] = '|';
            } else {
                mat[pos.0 as usize][pos.1 as usize] = '-';
            }
        } else {
            mat[pos.0 as usize][pos.1 as usize] = '+';
        }
        navigate(mat, pos, new_direction)
    } else {
        let cur_char = mat[pos.0 as usize][pos.1 as usize];
        if cur_char != '-' && cur_char != '|' {
            mat[pos.0 as usize][pos.1 as usize] = 'X';
        }
        navigate(mat, next_pos, cur_direction)
    }
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos: (i32, i32) = (0, 0);
    for (i, row) in mat.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == '^') {
            pos = (i as i32, j as i32);
            break;
        }
    }

    navigate(&mut mat, pos, Direction::North);

    let res = mat
        .iter()
        .map(|row| row.iter().filter(|&x| *x != '#' && *x != '.').count())
        .sum::<usize>() as i32;

    Ok(res)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos: (i32, i32) = (0, 0);
    for (i, row) in mat.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == '^') {
            pos = (i as i32, j as i32);
            break;
        }
    }

    let mut loop_count = 0;

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let mut mut_mat = mat.clone();
            if mut_mat[i][j] == '.' {
                mut_mat[i][j] = '#';
                let is_loop = navigate(&mut mut_mat, pos, Direction::North);
                if is_loop {
                    loop_count += 1;
                }
            }
        }
    }

    Ok(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"....#.....
                       .........#
                       ..........
                       ..#.......
                       .......#..
                       ..........
                       .#..^.....
                       ........#.
                       #.........
                       ......#..."#;

        let result = part1(input).unwrap();
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let input = r#"....#.....
                       .........#
                       ..........
                       ..#.......
                       .......#..
                       ..........
                       .#..^.....
                       ........#.
                       #.........
                       ......#..."#;

        let result = part2(input).unwrap();
        assert_eq!(result, 6);
    }
}
