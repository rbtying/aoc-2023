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
                if b2.z.end() < b.z.start() {
                    can_collide[b.label].insert(b2.label);
                }
                if b.z.end() < b2.z.start() {
                    can_collide[b2.label].insert(b.label);
                }
            }
        }
    }

    // Try to move every brick down
    for label in &brick_order {
        let max_z_under_brick = can_collide[label]
            .iter()
            .map(|l| *bricks[l].z.end())
            .filter(|z| z < bricks[label].z.start())
            .max()
            .unwrap_or(0);

        let delta = *bricks[label].z.start() - (max_z_under_brick + 1);
        let bz = bricks[label].z.start() - delta..=*bricks[label].z.end() - delta;
        bricks.get_mut(label).unwrap().z = bz;
    }

    let mut disintegrateable = 0;
    for removed in &brick_order {
        let mut d = true;

        for label in &brick_order {
            let max_z_under_brick = can_collide[label]
                .iter()
                .filter(|l| *l != removed)
                .map(|l| *bricks[l].z.end())
                .filter(|z| z < bricks[label].z.start())
                .max()
                .unwrap_or(0);

            let delta = *bricks[label].z.start() - (max_z_under_brick + 1);
            if delta > 0 {
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
                if b2.z.end() < b.z.start() {
                    can_collide[b.label].insert(b2.label);
                }
                if b.z.end() < b2.z.start() {
                    can_collide[b2.label].insert(b.label);
                }
            }
        }
    }
    // Try to move every brick down
    for label in &brick_order {
        let max_z_under_brick = can_collide[label]
            .iter()
            .map(|l| *bricks[l].z.end())
            .filter(|z| z < bricks[label].z.start())
            .max()
            .unwrap_or(0);

        let delta = *bricks[label].z.start() - (max_z_under_brick + 1);
        let bz = bricks[label].z.start() - delta..=*bricks[label].z.end() - delta;
        bricks.get_mut(label).unwrap().z = bz;
    }

    let mut sum = 0;
    for removed in &brick_order {
        let mut bricks = bricks.clone();
        let mut dropped = HashSet::new();
        let mut dirty = true;

        while dirty {
            dirty = false;

            // Try to move every brick down
            for label in &brick_order {
                let max_z_under_brick = can_collide[label]
                    .iter()
                    .filter(|l| *l != removed)
                    .map(|l| *bricks[l].z.end())
                    .filter(|z| z < bricks[label].z.start())
                    .max()
                    .unwrap_or(0);

                let delta = *bricks[label].z.start() - (max_z_under_brick + 1);
                let bz = bricks[label].z.start() - delta..=*bricks[label].z.end() - delta;
                if delta > 0 {
                    dropped.insert(label);
                    dirty = true;
                }
                bricks.get_mut(label).unwrap().z = bz;
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