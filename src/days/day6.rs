#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,

    Vertical,
    Horizontal,
    Both,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        matches!(
            self,
            Direction::North | Direction::South | Direction::Vertical | Direction::Both
        )
    }

    fn is_horizontal(&self) -> bool {
        matches!(
            self,
            Direction::East | Direction::West | Direction::Horizontal | Direction::Both
        )
    }

    fn get_next_turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
            Direction::Both => Direction::Both,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Char {
    Obstacle,
    Empty,
    Start,

    Visited(Direction),
}

impl Char {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Char::Obstacle,
            '.' => Char::Empty,
            '^' => Char::Start,
            _ => panic!("Invalid character"),
        }
    }

    fn mark_visited(&self, direction: Direction) -> Self {
        match self {
            Char::Visited(old_direction) => {
                if direction == *old_direction {
                    Char::Visited(direction)
                } else if direction.is_vertical() && old_direction.is_vertical() {
                    Char::Visited(Direction::Vertical)
                } else if direction.is_horizontal() && old_direction.is_horizontal() {
                    Char::Visited(Direction::Horizontal)
                } else {
                    Char::Visited(Direction::Both)
                }
            }
            _ => Char::Visited(direction),
        }
    }

    fn is_visited(&self, direction: Direction) -> bool {
        match self {
            Char::Visited(d) => {
                direction == *d
                    || (direction.is_vertical() && d.is_vertical())
                    || (direction.is_horizontal() && d.is_horizontal())
            }
            _ => false,
        }
    }
}

fn navigate(mat: &mut Vec<Vec<Char>>, pos: (i32, i32), cur_direction: Direction) -> bool {
    let next_pos = match cur_direction {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
        _ => panic!("Invalid direction"),
    };

    if next_pos.0 < 0
        || next_pos.0 >= mat.len() as i32
        || next_pos.1 < 0
        || next_pos.1 >= mat[0].len() as i32
    {
        mat[pos.0 as usize][pos.1 as usize] = Char::Visited(cur_direction);
        return false;
    }

    let next_char = mat[next_pos.0 as usize][next_pos.1 as usize];

    if next_char == Char::Obstacle {
        let new_direction = cur_direction.get_next_turn();
        let cur_char = mat[pos.0 as usize][pos.1 as usize];
        if cur_char.is_visited(cur_direction) {
            return true;
        }
        mat[pos.0 as usize][pos.1 as usize] = cur_char.mark_visited(cur_direction);
        navigate(mat, pos, new_direction)
    } else {
        let cur_char = mat[pos.0 as usize][pos.1 as usize];
        match cur_char {
            Char::Visited(_) => (),
            _ => {
                mat[pos.0 as usize][pos.1 as usize] = cur_char.mark_visited(cur_direction);
            }
        }

        navigate(mat, next_pos, cur_direction)
    }
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut mat = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(Char::from_char)
                .collect::<Vec<Char>>()
        })
        .collect::<Vec<Vec<Char>>>();

    let mut pos: (i32, i32) = (0, 0);
    for (i, row) in mat.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == Char::Start) {
            pos = (i as i32, j as i32);
            break;
        }
    }

    navigate(&mut mat, pos, Direction::North);

    let res = mat
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&x| *x != Char::Obstacle && *x != Char::Empty)
                .count()
        })
        .sum::<usize>() as i32;

    Ok(res)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let mat = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(Char::from_char)
                .collect::<Vec<Char>>()
        })
        .collect::<Vec<Vec<Char>>>();

    let mut pos: (i32, i32) = (0, 0);
    for (i, row) in mat.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == Char::Start) {
            pos = (i as i32, j as i32);
            break;
        }
    }

    let mut loop_count = 0;

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let mut mut_mat = mat.clone();
            if mut_mat[i][j] == Char::Empty {
                mut_mat[i][j] = Char::Obstacle;
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
