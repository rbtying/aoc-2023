use crate::prelude::*;

fn solve(input: &str, min_dist: i64, max_dist: i64) -> i64 {
    let grid = parse_char_grid(input);

    let start_pos = (0, 0);
    let (i_bounds, j_bounds) = get_grid_bounds(&grid);
    let dest_pos = (i_bounds.end - 1, j_bounds.end - 1);

    #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
    struct S {
        cost: i64,
        pos: (i64, i64),
        from_dir: (i64, i64),
    }

    let mut q = BinaryHeap::new();
    q.push(Reverse(S {
        cost: 0,
        pos: start_pos,
        from_dir: (0, 0),
    }));
    let mut dist = DefaultHashMap::new(std::i64::MAX);
    dist[(start_pos, (0, 0))] = 0;

    while let Some(Reverse(S {
        cost,
        pos,
        from_dir,
    })) = q.pop()
    {
        if pos == dest_pos {
            return cost;
        }
        // Compute forward edges
        for dir in FOUR_WAY {
            if dir == from_dir || dir == point_neg(from_dir) {
                continue;
            }

            let mut cost_sum = 0;
            for n in 1..=max_dist {
                let p = point_add(pos, point_mul(dir, n));
                let c = grid[p];

                if let Some(c) = c.to_digit(10) {
                    cost_sum += c as i64;
                    if n < min_dist {
                        continue;
                    }
                    let next = S {
                        cost: cost + cost_sum,
                        pos: p,
                        from_dir: dir,
                    };
                    if next.cost < dist[(next.pos, next.from_dir)] {
                        q.push(Reverse(next));
                        dist[(next.pos, next.from_dir)] = next.cost;
                    }
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 0, 3)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 102);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 907);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 94);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 1057);
    }
}
