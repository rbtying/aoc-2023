use crate::prelude::*;
use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;

fn get_loop(input: &str) -> Vec<(isize, isize)> {
    let grid = parse_char_grid(input);

    let (i_bounds, j_bounds) = get_grid_bounds(&grid);
    let mut s_pos = (-1, -1);
    let mut g = Graph::<(isize, isize), ()>::default();
    let mut nodes = HashMap::new();

    for i in i_bounds.clone() {
        for j in j_bounds.clone() {
            let n = *nodes.entry((i, j)).or_insert_with(|| g.add_node((i, j)));
            if grid[&(i, j)] == 'S' {
                s_pos = (i, j);
            }

            if "|-LJ7F".find(grid[&(i, j)]).is_some() {
                let edge = match grid[&(i, j)] {
                    '|' => ((i - 1, j), (i + 1, j)),
                    '-' => ((i, j - 1), (i, j + 1)),
                    'L' => ((i - 1, j), (i, j + 1)),
                    'J' => ((i - 1, j), (i, j - 1)),
                    '7' => ((i + 1, j), (i, j - 1)),
                    'F' => ((i + 1, j), (i, j + 1)),
                    _ => unreachable!(),
                };
                let a = *nodes.entry(edge.0).or_insert_with(|| g.add_node(edge.0));
                let b = *nodes.entry(edge.1).or_insert_with(|| g.add_node(edge.1));

                g.update_edge(n, a, ());
                g.update_edge(n, b, ());
            }
        }
    }

    let s = nodes[&s_pos];

    // Fix the start edges
    for e in g.edges(s).map(|e| e.id()).collect::<Vec<_>>() {
        g.remove_edge(e);
    }

    for n in g.node_indices().to_owned() {
        for nn in g.neighbors(n).collect::<Vec<_>>() {
            if nn == s {
                g.update_edge(s, n, ());
            }
        }
    }

    let mut stk = vec![(s, None)];
    let mut backtrack = HashMap::new();
    while let Some((curr, prev)) = stk.pop() {
        if curr == s && prev.is_some() {
            break;
        }
        for next in g.neighbors(curr) {
            if Some(next) == prev {
                continue;
            }
            stk.push((next, Some(curr)));
            backtrack.insert(next, curr);
        }
    }

    let mut path = vec![s];
    loop {
        let x = backtrack[&path.last().unwrap()];
        if x == s {
            break;
        }
        path.push(x);
    }

    path.into_iter()
        .map(|n| *g.node_weight(n).unwrap())
        .collect()
}

pub fn part1(input: &str) -> isize {
    let inloop = get_loop(input);
    (inloop.len() / 2) as isize
}

pub fn part2(input: &str) -> isize {
    let mut path = get_loop(input);
    let path_len = path.len() as isize;

    path.push(path[0]);

    // Abuse the shoelace theorem to find the total area, then remove the boundary.
    (path
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<isize>()
        - path_len)
        / 2
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
    const EXAMPLE2: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 6773);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 4);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 493);
    }
}
