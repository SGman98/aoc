pub fn part1(input: &str) -> Result<isize, &'static str> {
    let mut mat = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(Char::from_char)
                .collect::<Vec<Char>>()
        })
        .collect::<Vec<Vec<Char>>>();

    let mut pos: (isize, isize) = (0, 0);
    for (i, row) in mat.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == Char::Start) {
            pos = (i as isize, j as isize);
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
        .sum::<usize>() as isize;

    Ok(res)
}

pub fn part2(input: &str) -> Result<isize, &'static str> {
    let mat = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(Char::from_char)
                .collect::<Vec<Char>>()
        })
        .collect::<Vec<Vec<Char>>>();

    let mut pos: (isize, isize) = (0, 0);
    for (i, row) in mat.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == Char::Start) {
            pos = (i as isize, j as isize);
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

fn navigate(mat: &mut Vec<Vec<Char>>, pos: (isize, isize), cur_direction: Direction) -> bool {
    let next_pos = match cur_direction {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
        _ => panic!("Invalid direction"),
    };

    let cur_char = mat[pos.0 as usize][pos.1 as usize];
    mat[pos.0 as usize][pos.1 as usize] = cur_char.mark_visited(cur_direction);
    if cur_char.is_visited(cur_direction) {
        return true;
    }

    if next_pos.0 < 0
        || next_pos.0 >= mat.len() as isize
        || next_pos.1 < 0
        || next_pos.1 >= mat[0].len() as isize
    {
        return false;
    }

    let next_char = mat[next_pos.0 as usize][next_pos.1 as usize];

    if next_char == Char::Obstacle {
        navigate(mat, pos, cur_direction.get_next_turn())
    } else {
        navigate(mat, next_pos, cur_direction)
    }
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
            Direction::North | Direction::South | Direction::Vertical
        )
    }

    fn is_horizontal(&self) -> bool {
        matches!(
            self,
            Direction::East | Direction::West | Direction::Horizontal
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
            Char::Visited(d) => match direction {
                Direction::North => *d == Direction::North,
                Direction::South => *d == Direction::South,
                Direction::East => *d == Direction::East,
                Direction::West => *d == Direction::West,
                Direction::Vertical => d.is_vertical(),
                Direction::Horizontal => d.is_horizontal(),
                Direction::Both => true,
            },
            _ => false,
        }
    }
}
