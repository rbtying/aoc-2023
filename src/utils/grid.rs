use crate::prelude::*;

pub type IGrid2D = DefaultHashMap<(i64, i64), char>;
pub type Point = (i64, i64);

pub fn parse_char_grid(input: &str) -> IGrid2D {
    let mut g = IGrid2D::default();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            g.insert((i as i64, j as i64), ch);
        }
    }
    g
}

/// Returns (min_i..max_i+1), (min_j..max_j+1)
pub fn get_grid_bounds(g: &IGrid2D) -> (Range<i64>, Range<i64>) {
    let min = g
        .keys()
        .copied()
        .reduce(|(i, j), (ii, jj)| (i.min(ii), j.min(jj)))
        .unwrap();
    let max = g
        .keys()
        .copied()
        .reduce(|(i, j), (ii, jj)| (i.max(ii), j.max(jj)))
        .unwrap();

    (min.0..max.0 + 1, min.1..max.1 + 1)
}

pub fn rotate_grid_ccw(g: &IGrid2D) -> IGrid2D {
    let mut rotated = IGrid2D::default();
    let (i_bounds, _j_bounds) = get_grid_bounds(g);

    for (pt, c) in g.iter() {
        let pt = (
            i_bounds.end - pt.1 - 1, // -j => +i
            pt.0,                    // +i => +j
        );
        rotated.insert(pt, *c);
    }

    rotated
}

pub fn rotate_grid_cw(g: &IGrid2D) -> IGrid2D {
    let mut rotated = IGrid2D::default();
    let (i_bounds, _j_bounds) = get_grid_bounds(g);

    for (pt, c) in g.iter() {
        let pt = (
            pt.1,                    // +j => +i
            i_bounds.end - pt.0 - 1, // -i => +j
        );
        rotated.insert(pt, *c);
    }

    rotated
}

pub fn rotate_grid_inplace_cw(g: &mut IGrid2D) {
    let (i_bounds, _j_bounds) = get_grid_bounds(g);

    for (pt, c) in g.drain().collect::<Vec<_>>() {
        let pt = (
            pt.1,                    // +j => +i
            i_bounds.end - pt.0 - 1, // -i => +j
        );
        g.insert(pt, c);
    }
}

pub fn rotate_grid_inplace_ccw(g: &mut IGrid2D) {
    let (i_bounds, _j_bounds) = get_grid_bounds(g);

    for (pt, c) in g.drain().collect::<Vec<_>>() {
        let pt = (
            i_bounds.end - pt.1 - 1, // -j => +i
            pt.0,                    // +i => +j
        );
        g.insert(pt, c);
    }
}

pub fn get_grid_row(g: &IGrid2D, i: i64) -> impl Iterator<Item = char> + '_ {
    let (_, j_bounds) = get_grid_bounds(g);

    j_bounds.into_iter().map(move |j| g[&(i, j)])
}

pub fn get_grid_col(g: &IGrid2D, j: i64) -> impl Iterator<Item = char> + '_ {
    let (i_bounds, _) = get_grid_bounds(g);

    i_bounds.into_iter().map(move |i| g[&(i, j)])
}

pub fn print_char_grid(g: &IGrid2D) {
    let (i_bounds, j_bounds) = get_grid_bounds(g);
    for i in i_bounds {
        for j in j_bounds.clone() {
            let ch = g[&(i, j)];
            if ch == '\0' {
                eprint!(" ");
            } else {
                eprint!("{}", ch)
            }
        }
        eprintln!()
    }
}

pub fn print_char_grid_with<F: Fn((i64, i64), char) -> D, D: std::fmt::Display>(
    g: &IGrid2D,
    fmt: F,
) {
    let (i_bounds, j_bounds) = get_grid_bounds(g);
    for i in i_bounds {
        for j in j_bounds.clone() {
            let ch = g[&(i, j)];
            eprint!("{}", fmt((i, j), ch));
        }
        eprintln!()
    }
}

pub const fn point_add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}
pub const fn point_neg(a: (i64, i64)) -> (i64, i64) {
    (-a.0, -a.1)
}
pub const fn point_mul(a: (i64, i64), k: i64) -> (i64, i64) {
    (a.0 * k, a.1 * k)
}
pub const fn point_dot(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.0 + a.1 * b.1
}
pub const fn point_det(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

pub const UP: (i64, i64) = (-1, 0);
pub const DOWN: (i64, i64) = point_neg(UP);
pub const LEFT: (i64, i64) = (0, -1);
pub const RIGHT: (i64, i64) = point_neg(LEFT);
pub const UPLEFT: (i64, i64) = point_add(UP, LEFT);
pub const UPRIGHT: (i64, i64) = point_add(UP, RIGHT);
pub const DOWNLEFT: (i64, i64) = point_add(DOWN, LEFT);
pub const DOWNRIGHT: (i64, i64) = point_add(DOWN, RIGHT);

pub const FOUR_WAY: [(i64, i64); 4] = [LEFT, UP, RIGHT, DOWN];

pub const EIGHT_WAY: [(i64, i64); 8] =
    [LEFT, UPLEFT, UP, UPRIGHT, RIGHT, DOWNRIGHT, DOWN, DOWNLEFT];

pub fn adjacents(
    (i, j): (i64, i64),
    deltas: impl IntoIterator<Item = (i64, i64)>,
) -> impl Iterator<Item = (i64, i64)> {
    deltas.into_iter().map(move |(di, dj)| (i + di, j + dj))
}
