use crate::prelude::*;

fn fall_north(grid: &mut IGrid2D) {
    let mut pts = grid.drain().collect::<Vec<_>>();
    pts.sort();

    for (mut p, c) in pts {
        if c == '#' {
            grid.insert(p, '#');
        } else if c == 'O' {
            while grid[&(p.0 - 1, p.1)] == '.' || grid[&(p.0 - 1, p.1)] == '\0' && p.0 > 0 {
                p.0 -= 1;
            }
            grid.insert(p, 'O');
        }
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
    let mut g = parse_char_grid(input);
    fall_north(&mut g);
    score(&g)
}

fn score(g: &IGrid2D) -> i64 {
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

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
    let mut g = parse_char_grid(input);
    g.retain(|_, v| *v != '.');

    let res = find_cycle_equals(
        g.clone(),
        |g2| {
            for _ in 0..4 {
                fall_north(g2);
                rotate_grid_inplace_cw(g2);
            }
        },
        |g2| {
            let mut h = FnvHasher::default();
            g2.hash(&mut h);
            (h.finish(), score(g2))
        },
    );
    res[1000000000 + 1].1
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
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 64);
    }
}
