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

// Apply shoelace theorem -- returns signed area.
pub fn compute_lattice_polygon_area(mut points: Vec<Point>) -> isize {
    if points[0] != *points.last().unwrap() {
        points.push(points[0]);
    }
    points
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<isize>()
        / 2
}

// Apply Pick's theroem
pub fn total_lattice_polygon_area_from_interior_boundary(
    interior_area: isize,
    boundary_area: isize,
) -> isize {
    interior_area.abs() + boundary_area.abs() / 2 + 1
}

// Apply Pick's theroem
pub fn interior_lattice_polygon_area_from_total_boundary(
    total_area: isize,
    boundary_area: isize,
) -> isize {
    total_area.abs() - boundary_area.abs() / 2 + 1
}
