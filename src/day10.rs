use crate::prelude::*;

fn get_loop(input: &str) -> Vec<(isize, isize)> {
    let grid = parse_char_grid(input);

    let (i_bounds, j_bounds) = get_grid_bounds(&grid);
    let mut s_pos = (-1, -1);
    let mut g = Graph::<(isize, isize), ()>::default();
    let mut nodes = HashMap::new();

    for i in i_bounds.clone() {
        for j in j_bounds.clone() {
            let p = (i, j);
            let n = *nodes.entry(p).or_insert_with(|| g.add_node(p));
            if grid[&p] == 'S' {
                s_pos = (i, j);
            }

            if "|-LJ7F".find(grid[&(i, j)]).is_some() {
                let exits = match grid[&p] {
                    '|' => [UP, DOWN],
                    '-' => [LEFT, RIGHT],
                    'L' => [UP, RIGHT],
                    'J' => [UP, LEFT],
                    '7' => [LEFT, DOWN],
                    'F' => [RIGHT, DOWN],
                    _ => unreachable!(),
                };

                for d in exits {
                    let dst = point_add(p, d);
                    let node_dst = *nodes.entry(dst).or_insert_with(|| g.add_node(dst));
                    g.update_edge(n, node_dst, ());
                }
            }
        }
    }

    let s = nodes[&s_pos];
    let into_s_orig = g.neighbors_directed(s, Incoming).collect::<Vec<_>>();
    for n in into_s_orig {
        g.update_edge(s, n, ());
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
    let path = get_loop(input);
    let path_len = path.len() as isize;

    let total_area = compute_lattice_polygon_area(path);
    interior_lattice_polygon_area_from_total_boundary(total_area, path_len as isize)
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
