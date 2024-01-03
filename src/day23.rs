use crate::prelude::*;

#[aoc(day23, part1)]
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
    q.push_back((start_pos, 0, HashSet::<P>::default()));

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
        for next in adjacents(pos, dirs.iter().copied()) {
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
fn simplified_graph(g: &IGrid2D) -> HashMap<P, Vec<(P, i64)>> {
    let (i_bounds, j_bounds) = get_grid_bounds(g);
    let mut junctions = HashMap::default();

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
            let mut visited = HashSet::<P>::default();
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

#[aoc_generator(day23, part2)]
pub fn part2_gen(input: &str) -> (usize, usize, Vec<Vec<(usize, i64)>>) {
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

    let mut s = simplified_graph(&g);

    // Slightly trim the graph depth -- there's only one path to the end, so we
    // can collapse the end by one node.
    let mut paths_to_end = s
        .iter()
        .filter(|(_, v)| v.iter().any(|(a, _)| *a == end_pos))
        .collect::<Vec<_>>();
    assert_eq!(paths_to_end.len(), 1);
    let (new_end_pos, next) = paths_to_end.remove(0);
    let new_end_pos = *new_end_pos;
    let extra_cost = next
        .iter()
        .filter(|(p, _)| *p == end_pos)
        .map(|(_, c)| *c)
        .next()
        .unwrap();
    s.remove(&end_pos);
    for v in s.values_mut() {
        for (p, c) in v {
            if *p == new_end_pos {
                *c += extra_cost;
            }
        }
    }
    s.get_mut(&new_end_pos).unwrap().clear();
    end_pos = new_end_pos;

    // Rewrite the graph using integer nodes for faster runtime
    let keys = s.keys().collect::<Vec<_>>();
    let mut graf = Vec::with_capacity(keys.len());

    for k in &keys {
        let v = &s[k];
        graf.push(
            v.iter()
                .map(|(p, c)| (keys.iter().position(|x| *x == p).unwrap(), *c))
                .collect::<Vec<_>>(),
        );
    }

    let start = keys.iter().position(|x| **x == start_pos).unwrap();
    let end = keys.iter().position(|x| **x == end_pos).unwrap();

    (start, end, graf)
}

#[aoc(day23, part2)]
pub fn part2((start, end, graf): &(usize, usize, Vec<Vec<(usize, i64)>>)) -> i64 {
    fn dfs(
        s: &[Vec<(usize, i64)>],
        pos: usize,
        steps: i64,
        mut visited: u64,
        end_pos: usize,
    ) -> Option<i64> {
        if pos == end_pos {
            Some(steps)
        } else {
            visited |= 1 << pos;
            let mut b = None;
            for (next, st) in &s[pos] {
                if visited & (1 << *next) == 0 {
                    if let Some(v) = dfs(s, *next, steps + st, visited, end_pos) {
                        if b.map(|vv| vv < v).unwrap_or(true) {
                            b = Some(v);
                        }
                    }
                }
            }
            b
        }
    }

    dfs(graf, *start, 0, 0, *end).unwrap()
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
    fn part2_example() {
        assert_eq!(part2(&part2_gen(EXAMPLE)), 154);
    }
}
