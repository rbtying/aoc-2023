use crate::prelude::*;

pub fn part1(input: &str) -> u32 {
    let grid = parse_char_grid(input);
    let (i_range, j_range) = get_grid_bounds(&grid);

    let mut contiguous_number_starts = vec![];

    for i in i_range {
        let mut was_digit = false;
        for j in j_range.clone() {
            let ch = grid[&(i, j)];
            if !was_digit && ch.is_ascii_digit() {
                contiguous_number_starts.push((i, j));
            }
            was_digit = ch.is_ascii_digit();
        }
    }

    let mut sums = 0;
    for (i, j) in contiguous_number_starts {
        let mut s = 0;
        let mut is_adjacent = false;
        for j in j.. {
            let ch = grid[&(i, j)];
            if let Some(d) = ch.to_digit(10) {
                if adjacents((i, j), EIGHT_WAY).any(|pos| {
                    !grid[&pos].is_ascii_digit() && grid[&pos] != '\0' && grid[&pos] != '.'
                }) {
                    is_adjacent = true;
                }
                s = s * 10 + d;
            } else {
                break;
            }
        }
        if is_adjacent {
            sums += s;
        }
    }

    sums
}

pub fn part2(input: &str) -> u32 {
    let grid = parse_char_grid(input);
    let (i_range, j_range) = get_grid_bounds(&grid);

    let mut contiguous_number_starts = vec![];

    for i in i_range {
        let mut was_digit = false;
        for j in j_range.clone() {
            let ch = grid[&(i, j)];
            if !was_digit && ch.is_ascii_digit() {
                contiguous_number_starts.push((i, j));
            }
            was_digit = ch.is_ascii_digit();
        }
    }

    let mut adjacent_to_gears = HashMap::new();
    let mut values = HashMap::new();

    for (i, j) in contiguous_number_starts {
        let start = (i, j);
        let mut s = 0;
        for j in j.. {
            let ch = grid[&(i, j)];
            if let Some(d) = ch.to_digit(10) {
                for pos in adjacents((i, j), EIGHT_WAY) {
                    if grid[&pos] == '*' {
                        adjacent_to_gears
                            .entry(pos)
                            .or_insert_with(HashSet::new)
                            .insert(start);
                    }
                }
                s = s * 10 + d;
            } else {
                break;
            }
        }
        values.insert(start, s);
    }

    let mut sum = 0;

    for starts in adjacent_to_gears.values() {
        if starts.len() == 2 {
            let mut s = starts.iter();
            let gear_ratio = values[&s.next().unwrap()] * values[&s.next().unwrap()];
            sum += gear_ratio;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 4361);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 544664);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 467835);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 84495585);
    }
}
