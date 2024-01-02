use crate::prelude::*;

use smtlib::{backend::Z3Binary, Real, Solver, Sort};

fn intersects(
    (xa, ya, dxa, dya): (f64, f64, f64, f64),
    (xb, yb, dxb, dyb): (f64, f64, f64, f64),
) -> Option<(f64, f64)> {
    let ma = dya / dxa;
    let mb = dyb / dxb;
    if (mb - ma).abs() < f64::EPSILON {
        return None;
    }
    let x = (ma * xa - mb * xb + yb - ya) / (ma - mb);
    let y = (ma * mb * (xb - xa) + mb * ya - ma * yb) / (mb - ma);
    Some((x, y))
}

#[aoc(day24, part1)]
pub fn part1_in(input: &str) -> i64 {
    part1(input, 200000000000000., 400000000000000.)
}

pub fn part1(input: &str, min: f64, max: f64) -> i64 {
    let mut eqns = vec![];
    for line in input.lines() {
        let (pos, vel) = split1(line, " @ ");
        let pos = parse_floats(pos);
        let vel = parse_floats(vel);

        eqns.push((pos, vel));
    }

    let mut ct = 0;
    for (idx, eqn @ (pos, vel)) in eqns.iter().enumerate() {
        for eqn2 @ (pos2, vel2) in eqns.iter().skip(idx) {
            if eqn == eqn2 {
                continue;
            }

            if let Some((x, y)) = intersects(
                (pos[0], pos[1], vel[0], vel[1]),
                (pos2[0], pos2[1], vel2[0], vel2[1]),
            ) {
                if (vel[0] < 0. && x > pos[0]) || (vel[0]) > 0. && x < pos[0] {
                    continue;
                }
                if (vel2[0] < 0. && x > pos2[0]) || (vel2[0] > 0. && x < pos2[0]) {
                    continue;
                }

                if min <= x && x <= max && min <= y && y <= max {
                    ct += 1;
                }
            }
        }
    }

    ct
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> u64 {
    let mut eqns = vec![];
    for line in input.lines().take(3) {
        let (pos, vel) = split1(line, " @ ");
        let pos = parse_floats(pos);
        let vel = parse_floats(vel);

        eqns.push((pos, vel));
    }

    let mut solver = Solver::new(Z3Binary::new("z3").unwrap()).unwrap();
    let x = Real::from_name("x");
    let y = Real::from_name("y");
    let z = Real::from_name("z");
    let dx = Real::from_name("dx");
    let dy = Real::from_name("dy");
    let dz = Real::from_name("dz");

    for (idx, (pos, vel)) in eqns.into_iter().enumerate() {
        let t = Real::from_name(format!("t{}", idx));
        solver.assert(t.gt(0)).unwrap();
        solver
            .assert((x + t * dx)._eq(pos[0] + t * vel[0]))
            .unwrap();
        solver
            .assert((y + t * dy)._eq(pos[1] + t * vel[1]))
            .unwrap();
        solver
            .assert((z + t * dz)._eq(pos[2] + t * vel[2]))
            .unwrap();
    }
    let model = solver.check_sat_with_model().unwrap().expect_sat().unwrap();
    let x: u64 = model.eval(x).unwrap().to_string().parse::<f64>().unwrap() as u64;
    let y: u64 = model.eval(y).unwrap().to_string().parse::<f64>().unwrap() as u64;
    let z: u64 = model.eval(z).unwrap().to_string().parse::<f64>().unwrap() as u64;
    x + y + z
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 7., 27.), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 47);
    }
}
