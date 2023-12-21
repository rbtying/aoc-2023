use crate::prelude::*;

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

pub fn part2(input: &str) -> i64 {
    let g = parse_char_grid(input);
    let (i_bounds, j_bounds) = get_grid_bounds(&g);

    let (start_pos, _) = g.iter().find(|(_, c)| **c == 'S').unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((*start_pos, 0));

    let mut visited: DefaultHashMap<i64, HashSet<(i64, i64)>> = DefaultHashMap::default();
    let mut counts = vec![];
    let mut idxes = vec![];

    let target: i64 = 26501365;

    while let Some((n, step)) = queue.pop_front() {
        if visited[step].contains(&n) {
            continue;
        }
        visited[step].insert(n);

        // if we got to `step`, we've finished `step-1`. Do some cleanup
        if !visited[step - 1].is_empty() {
            let prev_len = visited[step - 1].len() as i64;
            if ((step - 1) - i_bounds.end / 2) % i_bounds.end == 0 {
                idxes.push(step - 1);
                counts.push(prev_len);

                eprintln!("counts: {:?}", counts);
                if counts.len() == 3 {
                    return polynomial_regression(&idxes, &counts, counts.len() - 1).eval(target);
                }
            }
            visited.remove(&(step - 1));
        }

        if step == target {
            continue;
        }

        for x in adjacents(n, FOUR_WAY) {
            let xx = (x.0.rem_euclid(i_bounds.end), x.1.rem_euclid(j_bounds.end));

            if (g[xx] == '.' || g[xx] == 'S') && !visited[step + 1].contains(&x) {
                queue.push_back((x, step + 1));
            }
        }
    }
    print_char_grid(&g);

    visited[target].len() as i64
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

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!()), 64), 3578);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 594115391548176);
    }
}
