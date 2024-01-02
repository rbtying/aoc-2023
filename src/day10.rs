use petgraph::data::Build;

use crate::prelude::*;

fn get_loop(input: &str) -> Vec<(i64, i64)> {
    let grid = parse_char_grid(input);

    let (i_bounds, j_bounds) = get_grid_bounds(&grid);
    let mut s_pos = (-1, -1);
    let mut g = DiGraphMap::default();

    for i in i_bounds.clone() {
        for j in j_bounds.clone() {
            let p = (i, j);
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
                    g.update_edge(p, dst, ());
                }
            }
        }
    }

    let into_s_orig = g.neighbors_directed(s_pos, Incoming).collect::<Vec<_>>();
    for n in into_s_orig {
        g.update_edge(s_pos, n, ());
    }

    let mut stk = vec![(s_pos, None)];
    let mut backtrack = HashMap::default();
    while let Some((curr, prev)) = stk.pop() {
        if curr == s_pos && prev.is_some() {
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

    let mut path = vec![s_pos];
    loop {
        let x = backtrack[&path.last().unwrap()];
        if x == s_pos {
            break;
        }
        path.push(x);
    }

    path
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let inloop = get_loop(input);
    (inloop.len() / 2) as i64
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i64 {
    let path = get_loop(input);
    let path_len = path.len() as i64;

    let total_area = compute_lattice_polygon_area(path);
    // We want the interior area, not the total area of the polygon. We can
    // apply Pick's theorem to subtract out the boundary.
    total_area.abs() - (path_len) / 2 + 1
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
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 4);
    }
}
