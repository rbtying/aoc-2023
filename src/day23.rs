use crate::prelude::*;

pub fn part1(input: &str) -> i64 {
    let g = parse_char_grid(input);

    let (_, j_bounds) = get_grid_bounds(&g);

    let mut start_pos = (-1, -1);
    for j in j_bounds.clone() {
        if g[(0, j)] == '.' {
            start_pos = (0, j);
            break;
        }
    }

    let mut q = VecDeque::new();
    q.push_back((start_pos, 0, FnvHashSet::<P>::default()));

    let mut max_steps = 0;

    while let Some((pos, steps, mut visited)) = q.pop_back() {
        visited.insert(pos);
        if steps > max_steps {
            max_steps = steps;
        }
        let dirs = match g[pos] {
            '#' => continue,
            '.' => FOUR_WAY.as_slice(),
            '>' => [RIGHT].as_slice(),
            'v' => [DOWN].as_slice(),
            '^' => [UP].as_slice(),
            '<' => [LEFT].as_slice(),
            _ => unreachable!(),
        };
        for next in adjacents(pos, dirs.into_iter().copied()) {
            if ".>v^<".find(g[next]).is_some() && !visited.contains(&next) {
                q.push_back((next, steps + 1, visited.clone()));
            }
        }
    }

    max_steps
}

type P = (i64, i64);

/// Simplify the graph by following the hallways and making them into long
/// edges.
fn simplified_graph(g: &IGrid2D) -> FnvHashMap<P, Vec<(P, i64)>> {
    let (i_bounds, j_bounds) = get_grid_bounds(g);
    let mut junctions = FnvHashMap::default();

    for i in i_bounds.clone() {
        for j in j_bounds.clone() {
            let p = (i, j);
            if g[p] == '.' {
                let exits = adjacents(p, FOUR_WAY).filter(|n| g[n] == '.').count();
                if exits != 2 {
                    junctions.insert(p, vec![]);
                }
            }
        }
    }

    for j in junctions.keys().copied().collect::<Vec<_>>() {
        let mut edges = vec![];

        for n in adjacents(j, FOUR_WAY) {
            let mut visited = FnvHashSet::default();
            visited.insert(j);

            let mut q = vec![(n, 1)];

            while let Some((n, s)) = q.pop() {
                visited.insert(n);

                if junctions.contains_key(&n) {
                    edges.push((n, s));
                    break;
                }

                for nn in adjacents(n, FOUR_WAY) {
                    if !visited.contains(&nn) && g[nn] == '.' {
                        q.push((nn, s + 1));
                    }
                }
            }
        }

        junctions.get_mut(&j).unwrap().extend(edges);
    }

    junctions
}

pub fn part2(input: &str) -> i64 {
    let mut g = parse_char_grid(input);

    let (i_bounds, j_bounds) = get_grid_bounds(&g);

    let mut start_pos = (-1, -1);
    let mut end_pos = (-1, -1);
    for j in j_bounds.clone() {
        if g[(0, j)] == '.' {
            start_pos = (0, j);
        }
        if g[(i_bounds.end - 1, j)] == '.' {
            end_pos = (i_bounds.end - 1, j);
        }
    }

    for v in g.values_mut() {
        if ".>v^<".find(*v).is_some() {
            *v = '.';
        }
    }

    let s = simplified_graph(&g);

    fn dfs(
        s: &FnvHashMap<P, Vec<(P, i64)>>,
        pos: P,
        steps: i64,
        visited: &mut FnvHashSet<P>,
        end_pos: P,
    ) -> Option<i64> {
        if pos == end_pos {
            Some(steps)
        } else {
            visited.insert(pos);
            let mut b = None;
            for (next, st) in &s[&pos] {
                if !visited.contains(next) {
                    if let Some(v) = dfs(s, *next, steps + st, visited, end_pos) {
                        if b.map(|vv| vv < v).unwrap_or(true) {
                            b = Some(v);
                        }
                    }
                }
            }
            visited.remove(&pos);
            b
        }
    }

    dfs(&s, start_pos, 0, &mut FnvHashSet::default(), end_pos).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 94);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 2358);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 154);
    }

    #[ignore]
    #[test]
    fn part2_input() {
        // This takes like 2 minutes
        assert_eq!(part2(&read_day_input(std::module_path!())), 6586);
    }
}
