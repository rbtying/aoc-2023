use crate::prelude::*;

fn solve(g: &IGrid2D, initial_beams: Vec<((i64, i64), (i64, i64))>) -> i64 {
    let mut max_energized = -1;
    let (i_bounds, j_bounds) = get_grid_bounds(g);

    for beam in initial_beams {
        let mut beams = vec![beam];
        let mut visited = HashSet::default();

        while let Some((pos, mut dir)) = beams.pop() {
            visited.insert((pos, dir));

            let mut to_add = vec![];
            if "./\\".find(g[pos]).is_some() {
                if g[pos] == '.' {
                } else if g[pos] == '/' {
                    if dir == RIGHT {
                        dir = UP;
                    } else if dir == LEFT {
                        dir = DOWN;
                    } else if dir == UP {
                        dir = RIGHT;
                    } else if dir == DOWN {
                        dir = LEFT;
                    }
                } else if g[pos] == '\\' {
                    if dir == RIGHT {
                        dir = DOWN;
                    } else if dir == LEFT {
                        dir = UP;
                    } else if dir == UP {
                        dir = LEFT;
                    } else if dir == DOWN {
                        dir = RIGHT;
                    }
                }

                to_add.push((point_add(pos, dir), dir));
            } else if "-|".find(g[pos]).is_some() {
                if (g[pos] == '-' && (dir == RIGHT || dir == LEFT))
                    || (g[pos] == '|' && (dir == UP || dir == DOWN))
                {
                    to_add.push((point_add(pos, dir), dir));
                } else if g[pos] == '-' {
                    to_add.push((point_add(pos, LEFT), LEFT));
                    to_add.push((point_add(pos, RIGHT), RIGHT));
                } else if g[pos] == '|' {
                    to_add.push((point_add(pos, UP), UP));
                    to_add.push((point_add(pos, DOWN), DOWN));
                }
            }
            to_add.retain(|d @ ((i, j), _)| {
                !visited.contains(d)
                    && *i >= i_bounds.start
                    && *i < i_bounds.end
                    && *j >= j_bounds.start
                    && *j < j_bounds.end
            });
            beams.extend(to_add);
        }
        max_energized = max_energized.max(
            visited
                .into_iter()
                .map(|x| x.0)
                .collect::<HashSet<_>>()
                .len() as i64,
        )
    }
    max_energized
}

pub fn part1(input: &str) -> i64 {
    let g = parse_char_grid(input);
    solve(&g, vec![((0, 0), RIGHT)])
}

pub fn part2(input: &str) -> i64 {
    let g = parse_char_grid(input);
    let (i_bounds, j_bounds) = get_grid_bounds(&g);
    let mut initial_beams = vec![];

    for i in i_bounds.clone() {
        initial_beams.push(((i, 0), RIGHT));
        initial_beams.push(((i, j_bounds.end - 1), LEFT));
    }
    for j in j_bounds.clone() {
        initial_beams.push(((0, j), DOWN));
        initial_beams.push(((i_bounds.end - 1, j), UP));
    }

    solve(&g, initial_beams)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 46);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 8901);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 51);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 9064);
    }
}
