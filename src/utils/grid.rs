use std::ops::RangeInclusive;

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

/// Returns (min_i..=max_i), (min_j..=max_j)
pub fn get_grid_bounds(g: &IGrid2D) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
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

    (min.0..=max.0, min.1..=max.1)
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

pub const FOUR_WAY: [(isize, isize); 4] = [
    // left
    (0, -1),
    // top
    (-1, 0),
    // right
    (0, 1),
    // bottom
    (1, 0),
];

pub const EIGHT_WAY: [(isize, isize); 8] = [
    // left
    (0, -1),
    // topleft
    (-1, -1),
    // top
    (-1, 0),
    // topright
    (-1, 1),
    // right
    (0, 1),
    // bottomright
    (1, 1),
    // bottom
    (1, 0),
    // bottomleft
    (1, -1),
];

pub fn adjacents(
    (i, j): (isize, isize),
    deltas: impl IntoIterator<Item = (isize, isize)>,
) -> impl Iterator<Item = (isize, isize)> {
    deltas.into_iter().map(move |(di, dj)| (i + di, j + dj))
}
