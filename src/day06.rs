use crate::prelude::*;

fn solve_simple(time: isize, distance: isize) -> isize {
    (0..=time)
        .filter(|hold| hold * (time - hold) > distance)
        .count() as isize
}

pub fn part1(input: &str) -> isize {
    let (line1, line2) = split1(input, "\n");
    let times = split1(line1, ":")
        .1
        .split_whitespace()
        .map(parse1)
        .collect::<Vec<isize>>();
    let distances = split1(line2, ":")
        .1
        .split_whitespace()
        .map(parse1)
        .collect::<Vec<isize>>();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| solve_simple(time, distance))
        .product()
}

pub fn part2(input: &str) -> isize {
    let (line1, line2) = split1(input, "\n");
    let time: isize = parse1(&split1(line1, ":").1.replace(" ", ""));
    let distance: isize = parse1(&split1(line2, ":").1.replace(" ", ""));
    solve_simple(time, distance)
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
