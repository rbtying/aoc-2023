use crate::prelude::*;

pub fn draw_polygon(mut points: Vec<Point>) {
    let mut g = IGrid2D::default();

    if points[0] != *points.last().unwrap() {
        points.push(points[0]);
    }

    for w in points.windows(2) {
        let (mut from, mut to) = (w[0], w[1]);
        if to.0 < from.0 {
            std::mem::swap(&mut from, &mut to);
        }

        if to.0 == from.0 {
            // horizontal line
            if to.1 < from.1 {
                std::mem::swap(&mut from, &mut to);
            }
            for j in from.1..=to.1 {
                g[(from.0, j)] = '-';
            }
        } else if to.1 == from.1 {
            // vertical line
            for i in from.0..=to.0 {
                g[(i, from.1)] = '|';
            }
        } else {
            // try our best; diagonal line
            for i in from.0..=to.0 {
                let j = from.1 + ((to.1 - from.1) * (i - from.0) / (to.0 - from.0));
                g[(i, j)] = '.';
            }
        }
    }
    print_char_grid(&g);
}

/// Apply shoelace theorem -- returns signed area.
/// Often used in conjunction with Pick's theorem:
///
/// Total Area = Interior Area + (Boundary Area / 2) - 1
pub fn compute_lattice_polygon_area(mut points: Vec<Point>) -> i64 {
    if points[0] != *points.last().unwrap() {
        points.push(points[0]);
    }
    points
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<i64>()
        / 2
}
