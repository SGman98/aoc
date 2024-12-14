use std::collections::HashSet;

use itertools::{repeat_n, Itertools};

pub fn part1(input: &str) -> Result<isize, &'static str> {
    let mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let chars: HashSet<char> = mat
        .iter()
        .flat_map(|row| row.iter().filter(|&&x| x != '.'))
        .copied()
        .collect();

    let locations = chars
        .iter()
        .map(|&c| {
            (
                c,
                mat.iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(_, &x)| x == c)
                            .map(move |(j, _)| (i as isize, j as isize))
                    })
                    .collect::<Vec<(isize, isize)>>(),
            )
        })
        .collect::<HashSet<(char, Vec<(isize, isize)>)>>();

    let antinode_locations = locations
        .iter()
        .flat_map(|(cur_char, locations)| {
            repeat_n(locations.iter(), 2)
                .multi_cartesian_product()
                .flat_map(|pair| {
                    let (a, b) = (pair[0], pair[1]);
                    let (di, dj) = (a.0 - b.0, a.1 - b.1); // distance

                    let a_reflected = (a.0 + di, a.1 + dj);
                    let b_reflected = (b.0 - di, b.1 - dj);

                    vec![a_reflected, b_reflected]
                })
                .filter(|&(i, j)| {
                    i >= 0
                        && i < mat.len() as isize
                        && j >= 0
                        && j < mat[0].len() as isize
                        && mat[i as usize][j as usize] != *cur_char
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .collect::<HashSet<(isize, isize)>>();

    Ok(antinode_locations.len() as isize)
}

pub fn part2(input: &str) -> Result<isize, &'static str> {
    let mat = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let chars: HashSet<char> = mat
        .iter()
        .flat_map(|row| row.iter().filter(|&&x| x != '.'))
        .copied()
        .collect();

    let locations = chars
        .iter()
        .map(|&c| {
            (
                c,
                mat.iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(_, &x)| x == c)
                            .map(move |(j, _)| (i as isize, j as isize))
                    })
                    .collect::<Vec<(isize, isize)>>(),
            )
        })
        .collect::<HashSet<(char, Vec<(isize, isize)>)>>();

    let antinode_locations = locations
        .iter()
        .flat_map(|(cur_char, locations)| {
            repeat_n(locations.iter(), 2)
                .multi_cartesian_product()
                .flat_map(|pair| {
                    let mut a = *pair[0];
                    let mut b = *pair[1];

                    if a == b {
                        return vec![];
                    }
                    let (di, dj) = (a.0 - b.0, a.1 - b.1); // distance

                    let mut reflections = vec![];

                    loop {
                        a = (a.0 + di, a.1 + dj);
                        if !(a.0 >= 0
                            && a.0 < mat.len() as isize
                            && a.1 >= 0
                            && a.1 < mat[0].len() as isize)
                        {
                            break;
                        }
                        reflections.push(a);
                    }

                    loop {
                        b = (b.0 - di, b.1 - dj);
                        if !(b.0 >= 0
                            && b.0 < mat.len() as isize
                            && b.1 >= 0
                            && b.1 < mat[0].len() as isize)
                        {
                            break;
                        }
                        reflections.push(b);
                    }

                    reflections
                })
                .filter(|&(i, j)| {
                    i >= 0
                        && i < mat.len() as isize
                        && j >= 0
                        && j < mat[0].len() as isize
                        && mat[i as usize][j as usize] != *cur_char
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .merge(
            locations
                .clone()
                .into_iter()
                .flat_map(|(_, locations)| locations.into_iter())
                .collect::<Vec<(isize, isize)>>(),
        )
        .collect::<HashSet<(isize, isize)>>();


    Ok(antinode_locations.len() as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"............
                       ........0...
                       .....0......
                       .......0....
                       ....0.......
                       ......A.....
                       ............
                       ............
                       ........A...
                       .........A..
                       ............
                       ............"#;

        let result = part1(input).unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let input = r#"............
                       ........0...
                       .....0......
                       .......0....
                       ....0.......
                       ......A.....
                       ............
                       ............
                       ........A...
                       .........A..
                       ............
                       ............"#;

        let result = part2(input).unwrap();
        assert_eq!(result, 34);
    }
}
