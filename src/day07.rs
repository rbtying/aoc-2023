use crate::prelude::*;

fn count_and_type(input: &str) -> isize {
    let mut m = HashMap::new();
    for c in input.chars() {
        *m.entry(c).or_default() += 1;
    }

    let mut v = m.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|(_, ct)| -ct);
    type_(&v)
}

fn type_(hand: &[(char, isize)]) -> isize {
    match hand.len() {
        1 => 0,
        2 if hand[0].1 == 4 => 1,
        2 => 2,
        3 if hand[0].1 == 3 => 3,
        3 => 4,
        4 => 5,
        5 => 6,
        _ => unreachable!("bad {:?}", hand),
    }
}

fn char_(v: char) -> usize {
    "AKQJT98765432".find(v).unwrap()
}

pub fn part1(input: &str) -> isize {
    let mut rows = input
        .lines()
        .map(|line| split1(line, " "))
        .collect::<Vec<(&str, &str)>>();

    rows.sort_by(|a, b| {
        let v = count_and_type(a.0).cmp(&count_and_type(b.0));
        v.then_with(|| {
            a.0.chars()
                .zip(b.0.chars())
                .map(|(aa, bb)| char_(aa).cmp(&char_(bb)))
                .find(|c| *c != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
        .reverse()
    });

    rows.into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| parse1::<isize>(bid) * ((idx + 1) as isize))
        .sum::<isize>()
}

fn count_and_max_type_(input: &str) -> isize {
    assert_eq!(input.len(), 5);
    let mut m = HashMap::new();
    for c in input.chars() {
        *m.entry(c).or_default() += 1;
    }

    let mut v = m.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
    v.sort_by_key(|(_, ct)| -ct);

    if let Some(x) = m.remove(&'J') {
        if let Some(non_j) = v.iter().find(|x| x.0 != 'J') {
            *m.entry(non_j.0).or_default() += x;
        } else {
            m.insert('J', x);
        }
    }

    let mut v = m.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|(_, ct)| -ct);
    assert!(!v.is_empty());

    type_(&v)
}

fn char_2(v: char) -> usize {
    "AKQT98765432J".find(v).unwrap()
}

pub fn part2(input: &str) -> isize {
    let mut rows = input
        .lines()
        .map(|line| split1(line, " "))
        .collect::<Vec<(&str, &str)>>();

    rows.sort_by(|a, b| {
        let v = count_and_max_type_(a.0).cmp(&count_and_max_type_(b.0));
        v.then_with(|| {
            a.0.chars()
                .zip(b.0.chars())
                .map(|(aa, bb)| char_2(aa).cmp(&char_2(bb)))
                .find(|c| *c != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
        .reverse()
    });

    rows.into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| parse1::<isize>(bid) * ((idx + 1) as isize))
        .sum::<isize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 6440);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 247961593);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 5905);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 248750699);
    }
}
