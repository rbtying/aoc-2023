use crate::prelude::*;

pub type IGrid2D = DefaultHashMap<(isize, isize), char>;

pub fn parse_char_grid(input: &str) -> IGrid2D {
    let mut g = IGrid2D::default();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            g.insert((i as isize, j as isize), ch);
        }
    }
    g
}

/// Returns (min_i..max_i+1), (min_j..max_j+1)
pub fn get_grid_bounds(g: &IGrid2D) -> (Range<isize>, Range<isize>) {
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

pub fn get_grid_row(g: &IGrid2D, i: isize) -> impl Iterator<Item = char> + '_ {
    let (_, j_bounds) = get_grid_bounds(g);

    j_bounds.into_iter().map(move |j| g[&(i, j)])
}

pub fn get_grid_col(g: &IGrid2D, j: isize) -> impl Iterator<Item = char> + '_ {
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

pub const fn point_add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}
pub const fn point_neg(a: (isize, isize)) -> (isize, isize) {
    (-a.0, -a.1)
}
pub const fn point_mul(a: (isize, isize), k: isize) -> (isize, isize) {
    (a.0 * k, a.1 * k)
}
pub const fn point_dot(a: (isize, isize), b: (isize, isize)) -> isize {
    a.0 * b.0 + a.1 * b.1
}
pub const fn point_det(a: (isize, isize), b: (isize, isize)) -> isize {
    a.0 * b.1 - a.1 * b.0
}

pub const UP: (isize, isize) = (-1, 0);
pub const DOWN: (isize, isize) = point_neg(UP);
pub const LEFT: (isize, isize) = (0, -1);
pub const RIGHT: (isize, isize) = point_neg(LEFT);
pub const UPLEFT: (isize, isize) = point_add(UP, LEFT);
pub const UPRIGHT: (isize, isize) = point_add(UP, RIGHT);
pub const DOWNLEFT: (isize, isize) = point_add(DOWN, LEFT);
pub const DOWNRIGHT: (isize, isize) = point_add(DOWN, RIGHT);

pub const FOUR_WAY: [(isize, isize); 4] = [LEFT, UP, RIGHT, DOWN];

pub const EIGHT_WAY: [(isize, isize); 8] =
    [LEFT, UPLEFT, UP, UPRIGHT, RIGHT, DOWNRIGHT, DOWN, DOWNLEFT];

pub fn adjacents(
    (i, j): (isize, isize),
    deltas: impl IntoIterator<Item = (isize, isize)>,
) -> impl Iterator<Item = (isize, isize)> {
    deltas.into_iter().map(move |(di, dj)| (i + di, j + dj))
}
