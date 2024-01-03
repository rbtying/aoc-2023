use crate::prelude::*;

#[aoc(day21, part1)]
pub fn part1_in(input: &str) -> i64 {
    part1(input, 64)
}

pub fn part1(input: &str, target: i64) -> i64 {
    let g = parse_char_grid(input);

    let (start_pos, _) = g.iter().find(|(_, c)| **c == 'S').unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((*start_pos, target));

    let mut visited: DefaultHashMap<i64, HashSet<(i64, i64)>> = DefaultHashMap::default();

    while let Some((n, remaining)) = queue.pop_front() {
        if visited[remaining].contains(&n) {
            continue;
        }
        visited[remaining].insert(n);
        if remaining == 0 {
            continue;
        }

        for x in adjacents(n, FOUR_WAY) {
            if (g[x] == '.' || g[x] == 'S') && !visited[remaining - 1].contains(&x) {
                queue.push_back((x, remaining - 1));
            }
        }
    }

    visited[0].len() as i64
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    let g = parse_char_grid(input);
    let (i_bounds, j_bounds) = get_grid_bounds(&g);

    let (start_pos, _) = g.iter().find(|(_, c)| **c == 'S').unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((*start_pos, 0));

    let mut visited: DefaultHashMap<i64, HashSet<(i64, i64)>> = DefaultHashMap::default();

    visited[0].insert(*start_pos);

    let target: i64 = 26501365;
    let offset = target % i_bounds.end;

    while let Some((n, step)) = queue.pop_front() {
        // if we got to `step`, we've finished `step-1`. Do some cleanup
        if step == offset + 2 * i_bounds.end + 1 {
            break;
        }
        if step == target {
            continue;
        }

        for x in adjacents(n, FOUR_WAY) {
            let xx = (x.0.rem_euclid(i_bounds.end), x.1.rem_euclid(j_bounds.end));

            if visited[step + 1].is_empty() {
                let p = visited[step - 1].clone();
                visited.insert(step + 1, p);
            }

            if (g[xx] == '.' || g[xx] == 'S') && !visited[step + 1].contains(&x) {
                visited[step + 1].insert(x);
                queue.push_back((x, step + 1));
            }
        }
    }

    polynomial_regression(
        &[
            offset,
            offset + i_bounds.end,
            offset + i_bounds.end + i_bounds.end,
        ],
        &[
            visited[offset].len() as i64,
            visited[offset + i_bounds.end].len() as i64,
            visited[offset + i_bounds.end + i_bounds.end].len() as i64,
        ],
        2,
    )
    .eval(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 6), 16);
    }
}
