use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    label: usize,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

pub fn part1(input: &str) -> i64 {
    let mut bricks = HashMap::new();
    for (label, line) in input.lines().enumerate() {
        let (lhs, rhs) = split1(line, "~");
        let (x1, y1, z1): (i64, i64, i64) = parse3(lhs.split(','));
        let (x2, y2, z2): (i64, i64, i64) = parse3(rhs.split(','));

        bricks.insert(
            label,
            Brick {
                label,
                x: x1.min(x2)..=x1.max(x2),
                y: y1.min(y2)..=y1.max(y2),
                z: z1.min(z2)..=z1.max(z2),
            },
        );
    }

    let mut brick_order = bricks.keys().copied().collect::<Vec<_>>();
    brick_order.sort_by_key(|idx| *bricks[idx].z.start());

    let mut can_collide: DefaultHashMap<usize, HashSet<usize>> =
        DefaultHashMap::new(HashSet::default());

    for (i, b) in brick_order.iter().enumerate() {
        for b2 in brick_order.iter().skip(i) {
            let b = &bricks[b];
            let b2 = &bricks[b2];
            if b.label != b2.label
                && overlaps(b.x.clone(), b2.x.clone())
                && overlaps(b.y.clone(), b2.y.clone())
            {
                can_collide[b.label].insert(b2.label);
                can_collide[b2.label].insert(b.label);
            }
        }
    }

    // Try to move every brick down
    for label in &brick_order {
        loop {
            let bz = bricks[label].z.start() - 1..=*bricks[label].z.end() - 1;
            let mut can_go_down = true;
            for label2 in &can_collide[label] {
                let b2 = &bricks[label2];
                if overlaps(b2.z.clone(), bz.clone()) {
                    can_go_down = false;
                }
            }
            if can_go_down && *bz.start() > 0 {
                bricks.get_mut(label).unwrap().z = bz;
            } else {
                break;
            }
        }
    }

    let mut disintegrateable = 0;
    for label in &brick_order {
        let mut d = true;

        for label2 in &can_collide[label] {
            let bz2 = bricks[label2].z.start() - 1..=bricks[label2].z.end() - 1;

            if *bz2.start() == 0 {
                continue;
            }

            let mut can_go_down = true;
            for label3 in &can_collide[label2] {
                if label3 == label {
                    continue;
                }
                if overlaps(bricks[label3].z.clone(), bz2.clone()) {
                    can_go_down = false;
                }
            }
            if can_go_down {
                d = false;
                break;
            }
        }

        if d {
            disintegrateable += 1;
        }
    }

    disintegrateable
}

pub fn part2(input: &str) -> i64 {
    let mut bricks = HashMap::new();
    for (label, line) in input.lines().enumerate() {
        let (lhs, rhs) = split1(line, "~");
        let (x1, y1, z1): (i64, i64, i64) = parse3(lhs.split(','));
        let (x2, y2, z2): (i64, i64, i64) = parse3(rhs.split(','));

        bricks.insert(
            label,
            Brick {
                label,
                x: x1.min(x2)..=x1.max(x2),
                y: y1.min(y2)..=y1.max(y2),
                z: z1.min(z2)..=z1.max(z2),
            },
        );
    }

    let mut brick_order = bricks.keys().copied().collect::<Vec<_>>();
    brick_order.sort_by_key(|idx| *bricks[idx].z.start());

    let mut can_collide: DefaultHashMap<usize, HashSet<usize>> =
        DefaultHashMap::new(HashSet::default());

    for (i, b) in brick_order.iter().enumerate() {
        for b2 in brick_order.iter().skip(i) {
            let b = &bricks[b];
            let b2 = &bricks[b2];
            if b.label != b2.label
                && overlaps(b.x.clone(), b2.x.clone())
                && overlaps(b.y.clone(), b2.y.clone())
            {
                can_collide[b.label].insert(b2.label);
                can_collide[b2.label].insert(b.label);
            }
        }
    }

    // Try to move every brick down
    for label in &brick_order {
        loop {
            let bz = bricks[label].z.start() - 1..=*bricks[label].z.end() - 1;
            let mut can_go_down = true;
            for label2 in &can_collide[label] {
                let b2 = &bricks[label2];
                if overlaps(b2.z.clone(), bz.clone()) {
                    can_go_down = false;
                }
            }
            if can_go_down && *bz.start() > 0 {
                bricks.get_mut(label).unwrap().z = bz;
            } else {
                break;
            }
        }
    }

    let mut sum = 0;
    for removed in &brick_order {
        let mut bricks = bricks.clone();
        let mut dropped = HashSet::new();
        let mut dirty = true;

        while dirty {
            dirty = false;

            for label in &brick_order {
                loop {
                    let bz = bricks[label].z.start() - 1..=*bricks[label].z.end() - 1;
                    let mut can_go_down = true;
                    for label2 in &can_collide[label] {
                        if label2 == removed {
                            continue;
                        }
                        let b2 = &bricks[label2];
                        if overlaps(b2.z.clone(), bz.clone()) {
                            can_go_down = false;
                        }
                    }
                    if can_go_down && *bz.start() > 0 {
                        dirty = true;
                        dropped.insert(label);
                        bricks.get_mut(label).unwrap().z = bz;
                    } else {
                        break;
                    }
                }
            }
        }

        sum += dropped.len() as i64
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 432);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 7);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 63166);
    }
}
