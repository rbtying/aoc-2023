use crate::prelude::*;

fn hash_fn(s: &str) -> isize {
    let mut current_value = 0;
    for b in s.bytes() {
        current_value += b as isize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

pub fn part1(input: &str) -> isize {
    let mut sum = 0;
    for s in input.replace('\n', "").split(',') {
        sum += hash_fn(s);
    }
    sum
}

pub fn part2(input: &str) -> isize {
    let mut boxes: Vec<Vec<(String, isize)>> = vec![vec![]; 256];

    for s in input.replace('\n', "").split(',') {
        if s.contains('-') {
            let label = &s[0..s.len() - 1];
            let h = hash_fn(label) as usize;
            let v = &mut boxes[h];
            if let Some(p) = v.iter().position(|s| s.0 == label) {
                v.remove(p);
            }
        } else {
            let (label, focal): (&str, isize) = parse_split_once(s, "=");
            let h = hash_fn(label) as usize;
            let v = &mut boxes[h];
            if let Some(p) = v.iter().position(|s| s.0 == label) {
                v[p].1 = focal;
            } else {
                v.push((label.to_string(), focal));
            }
        }
    }

    let mut s = 0;

    for (slot, v) in boxes.iter().enumerate() {
        for (idx, (_, lens)) in v.iter().enumerate() {
            s += ((slot + 1) as isize) * ((idx + 1) as isize) * lens;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1320);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 510792);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 145);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 269410);
    }
}
