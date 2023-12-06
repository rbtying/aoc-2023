use crate::prelude::*;

fn solve_simple(time: isize, distance: isize) -> isize {
    (0..=time)
        .filter(|hold| hold * (time - hold) > distance)
        .count() as isize
}

// We can directly solve this quadratic equation using the quadratic formula....
// But the input was actually brute-force-able and there was no need to do so.
fn solve_quad(time: isize, distance: isize) -> isize {
    let a = -1_f64;
    let b = time as f64;
    let c = -distance as f64;

    let d = (b * b - 4. * a * c).sqrt();
    let x1 = (-b + d) / 2.;
    let x2 = (-b - d) / 2.;
    (x1 - x2) as isize
}

pub fn part1(input: &str) -> isize {
    let (line1, line2) = split1(input, "\n");
    let times = parse_ints(line1);
    let distances = parse_ints(line2);

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| solve_simple(time, distance))
        .product()
}

pub fn part2(input: &str) -> isize {
    let (line1, line2) = split1(input, "\n");
    let time: isize = parse_ints(&line1.replace(' ', ""))[0];
    let distance: isize = parse_ints(&line2.replace(' ', ""))[0];
    solve_quad(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 288);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 3316275);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 71503);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 27102791);
    }
}
