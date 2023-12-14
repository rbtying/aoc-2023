use crate::prelude::*;

fn fall_north(grid: &IGrid2D) -> IGrid2D {
    let mut res = IGrid2D::default();

    let mut pts = grid.keys().copied().collect::<Vec<_>>();
    pts.sort();

    for mut p in pts {
        if grid[&p] == '#' {
            res.insert(p, '#');
        } else if grid[&p] == 'O' {
            while res[&(p.0 - 1, p.1)] == '.' || res[&(p.0 - 1, p.1)] == '\0' && p.0 > 0 {
                p.0 -= 1;
            }
            res.insert(p, 'O');
        }
    }

    res
}

pub fn part1(input: &str) -> isize {
    let g = parse_char_grid(input);
    let g = fall_north(&g);
    score(&g)
}

fn score(g: &IGrid2D) -> isize {
    let mut sum = 0;
    // score
    let (i_bounds, _) = get_grid_bounds(g);

    for (pt, ch) in g.iter() {
        if *ch == 'O' {
            sum += i_bounds.end - pt.0;
        }
    }
    sum
}

pub fn part2(input: &str) -> isize {
    let mut g = parse_char_grid(input);
    g.retain(|_, v| *v != '.');
    let mut h = vec![];

    let target = 1000000000;
    for cycle in 0..target {
        for _ in 0..4 {
            g = fall_north(&g);
            g = rotate_grid_cw(&g);
        }
        for (offset, g2) in h.iter().enumerate() {
            if g == *g2 {
                // Found recurrence
                let n = cycle - offset;
                // solve target = offset + recurrence * n + offset2
                // target - offset = recurrence * n + offset2
                let offset2 = (target - offset) % n;
                let t = &h[offset + offset2 - 1];
                // score
                return score(t);
            }
        }

        h.push(g.clone());
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 136);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 106378);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 64);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 90795);
    }
}
