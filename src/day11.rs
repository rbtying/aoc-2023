use crate::prelude::*;

fn solve(input: &str, mul: i64) -> i64 {
    let orig_grid = parse_char_grid(input);
    let (i_bounds, j_bounds) = get_grid_bounds(&orig_grid);

    let mut galaxy_rows = HashSet::default();
    let mut galaxy_cols = HashSet::default();

    for i in i_bounds.clone() {
        for j in j_bounds.clone() {
            if orig_grid[&(i, j)] == '#' {
                galaxy_rows.insert(i);
                galaxy_cols.insert(j);
            }
        }
    }

    let mut galaxies = vec![];
    let mut y = 0;
    for i in i_bounds {
        if !galaxy_rows.contains(&i) {
            y += mul - 1;
        }
        let mut x = 0;
        for j in j_bounds.clone() {
            if !galaxy_cols.contains(&j) {
                x += mul - 1;
            }
            if orig_grid[&(i, j)] == '#' {
                galaxies.push((y, x));
            }
            x += 1;
        }
        y += 1;
    }

    let mut s = 0;

    for (idx, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(idx) {
            if g1 != g2 {
                let d = g1.0.max(g2.0) - g1.0.min(g2.0) + g1.1.max(g2.1) - g1.1.min(g2.1);
                s += d;
            }
        }
    }

    s
}

pub fn part1(input: &str) -> i64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 374);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 9556712);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 82000210);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 678626199476);
    }
}
