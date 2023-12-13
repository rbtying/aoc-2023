use crate::prelude::*;

fn solve(input: &str, smudges: isize) -> isize {
    let mut s = 0;
    for pattern in input.split("\n\n") {
        let grid = parse_char_grid(pattern);
        // find a perfect reflection across either a horizontal line between two rows or vertical between two columns

        let (i_bounds, j_bounds) = get_grid_bounds(&grid);

        for i in 1..i_bounds.end {
            let n = i.min(i_bounds.end - i - 1);
            let mut mismatches = 0;
            for r in 0..=n {
                for (a, b) in get_grid_row(&grid, i - r - 1).zip(get_grid_row(&grid, i + r)) {
                    if a != b && a != '\0' && b != '\0' {
                        mismatches += 1;
                    }
                }
            }
            if mismatches == smudges {
                s += 100 * i;
            }
        }

        for j in 1..j_bounds.end {
            let n = j.min(j_bounds.end - j - 1);
            let mut mismatches = 0;
            for r in 0..=n {
                for (a, b) in get_grid_col(&grid, j - r - 1).zip(get_grid_col(&grid, j + r)) {
                    if a != b && a != '\0' && b != '\0' {
                        mismatches += 1;
                    }
                }
            }
            if mismatches == smudges {
                s += j;
            }
        }
    }
    s
}

pub fn part1(input: &str) -> isize {
    solve(input, 0)
}

pub fn part2(input: &str) -> isize {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 405);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 35538);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 400);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 30442);
    }
}
