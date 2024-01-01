use crate::prelude::*;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (_, line) = split1(line, ": ");
        let (lhs, rhs) = split1(line, " | ");
        let winners: HashSet<usize> = lhs.split_whitespace().map(parse1).collect();
        let mine: HashSet<usize> = rhs.split_whitespace().map(parse1).collect();

        let num_winner = mine.intersection(&winners).count();

        if num_winner > 0 {
            sum += 2usize.pow((num_winner - 1) as u32);
        }
    }

    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut num_cards_processed = 0;
    let mut num_duplicates: DefaultHashMap<usize, usize> = DefaultHashMap::default();

    for (card_num, line) in input.lines().enumerate() {
        let card_num = card_num + 1;
        let (_, line) = split1(line, ": ");
        let (lhs, rhs) = split1(line, " | ");
        let winners: HashSet<usize> = lhs.split_whitespace().map(parse1).collect();
        let mine: HashSet<usize> = rhs.split_whitespace().map(parse1).collect();

        let num_winner = mine.intersection(&winners).count();
        for dx in 1..=num_winner {
            num_duplicates[card_num + dx] += num_duplicates[card_num] + 1;
        }
        num_cards_processed += 1 + num_duplicates[card_num];
    }

    num_cards_processed
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 30);
    }
}
