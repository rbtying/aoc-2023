use std::str::FromStr;

#[derive(Debug)]
struct Game {
    number: u32,
    views: Vec<View>,
}

impl Game {
    fn from_str(s: &str) -> Game {
        let line = s.split_once("Game ").unwrap().1;
        let (game_num, line) = line.split_once(": ").unwrap();
        Game {
            number: u32::from_str(game_num).unwrap(),
            views: line.split("; ").map(View::from_str).collect(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct View {
    red: u32,
    green: u32,
    blue: u32,
}

impl View {
    fn from_str(s: &str) -> View {
        let mut v = View::default();
        for segment in s.split(", ") {
            let (num, color) = segment.split_once(" ").unwrap();
            let num = u32::from_str(num).unwrap();
            match color {
                "red" => v.red = num,
                "blue" => v.blue = num,
                "green" => v.green = num,
                _ => unreachable!(),
            }
        }
        v
    }
}

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::from_str(line);

        let mut valid = true;
        for v in &game.views {
            if v.red > 12 || v.green > 13 || v.blue > 14 {
                valid = false;
            }
        }

        if valid {
            sum += game.number;
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::from_str(line);
        let mut max = View::default();

        for v in &game.views {
            max.red = v.red.max(max.red);
            max.blue = v.blue.max(max.blue);
            max.green = v.green.max(max.green);
        }

        sum += max.red * max.blue * max.green;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    const INPUT: &str = include_str!("../puzzle/day02/input");

    #[test]
    fn day01_part1_example() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    fn day01_part1_input() {
        assert_eq!(part1(INPUT), 2563);
    }

    #[test]
    fn day01_part2_example() {
        assert_eq!(part2(EXAMPLE), 2286);
    }

    #[test]
    fn day01_part2_input() {
        assert_eq!(part2(INPUT), 70768);
    }
}
