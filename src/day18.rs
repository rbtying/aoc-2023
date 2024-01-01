use crate::prelude::*;

fn solve(iter: impl IntoIterator<Item = (Point, i64)>) -> i64 {
    let mut pos = (0, 0);
    let mut path = vec![pos];
    let mut b = 0;

    for (dir, len) in iter {
        b += len;
        pos = point_add(pos, point_mul(dir, len));
        path.push(pos);
    }

    let interior_area = compute_lattice_polygon_area(path);
    // Because the problem specifies the dig kerf as 1mx1m blocks, we actually
    // are looking for the area of the polygon that is described by dilating the
    // existing boundary by 0.5m.
    //
    // As it happens, Pick's theorem describes how to remove that dilated area
    // (i.e. the extra width of the boundary) to get the area actually bounded
    // by the points, so we can invert the calculation to get the requested
    // area.
    interior_area.abs() + b / 2 + 1
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input.lines().map(|line| {
        let (dir, len, _): (&str, i64, &str) = parse3(line.split(' '));
        (
            match dir {
                "R" => RIGHT,
                "U" => UP,
                "D" => DOWN,
                "L" => LEFT,
                _ => unreachable!(),
            },
            len,
        )
    }))
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input.lines().map(|line| {
        let (_, _, color): (&str, i64, &str) = parse3(line.split(' '));
        let color = color.trim_matches(|c| "()#".find(c).is_some());
        let len = i64::from_str_radix(&color[0..color.len() - 1], 16).unwrap();

        (
            match color.chars().last().unwrap() {
                '0' => RIGHT,
                '1' => DOWN,
                '2' => LEFT,
                '3' => UP,
                _ => unreachable!(),
            },
            len,
        )
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 952408144115);
    }
}
