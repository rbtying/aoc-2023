pub fn part1(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let mut vs = vec![];
        for ch in line.chars() {
            if let Some(d) = ch.to_digit(10) {
                vs.push(d);
            }
        }
        s += vs[0] * 10 + vs[vs.len() - 1];
    }
    s
}

fn str_to_v(s: &str) -> Option<u32> {
    if let Some(d) = s.chars().next().unwrap().to_digit(10) {
        Some(d)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else if s.starts_with("zero") {
        Some(0)
    } else {
        None
    }
}

pub fn part2(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let mut vs = vec![];
        for (c, _) in line.char_indices() {
            let ss = line.split_at(c).1;
            if let Some(d) = str_to_v(ss) {
                vs.push(d);
            }
        }
        s += vs[0] * 10 + vs[vs.len() - 1];
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    const EXAMPLE2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    const INPUT: &str = include_str!("../puzzle/day01/input");

    #[test]
    fn day01_part1_example() {
        assert_eq!(part1(EXAMPLE), 142);
    }

    #[test]
    fn day01_part1_input() {
        assert_eq!(part1(INPUT), 54159);
    }

    #[test]
    fn day01_part2_example() {
        assert_eq!(part2(EXAMPLE2), 281);
    }

    #[test]
    fn day01_part2_input() {
        assert_eq!(part2(INPUT), 53866);
    }
}
