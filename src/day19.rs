use crate::prelude::*;

fn parse_rules(rules_str: &str) -> HashMap<String, Vec<(String, char, i64, String)>> {
    let mut rules = HashMap::new();
    for r in rules_str.lines() {
        let mut rr = vec![];
        let (name, r) = split1(r, "{");
        for s in r.trim_end_matches(|c| c == '}').split(',') {
            if s.contains('<') {
                let (expr, res) = split1(s, ":");
                let (var, val): (&str, i64) = parse_split_once(expr, "<");
                rr.push((var.to_string(), '<', val, res.to_string()));
            } else if s.contains('>') {
                let (expr, res) = split1(s, ":");
                let (var, val): (&str, i64) = parse_split_once(expr, ">");
                rr.push((var.to_string(), '>', val, res.to_string()));
            } else {
                rr.push((s.to_string(), '.', 0, s.to_string()));
            }
        }
        rules.insert(name.to_string(), rr);
    }
    rules
}

pub fn part1(input: &str) -> i64 {
    let (rules_str, parts_str) = split1(input, "\n\n");

    let rules = parse_rules(rules_str);

    let mut parts = vec![];
    for p in parts_str.lines() {
        let p = p.trim_matches(|c| c == '{' || c == '}');
        let mut d = HashMap::new();
        let pairs = p.split(',');

        for pair in pairs {
            let (label, val): (&str, i64) = parse_split_once(pair, "=");
            d.insert(label, val);
        }
        parts.push(d);
    }

    let mut sum = 0;
    for p in parts {
        let mut wf = "in";
        'eval: while wf != "R" && wf != "A" {
            let exprs = &rules[wf];
            for (var, op, val, res) in exprs {
                match op {
                    '<' if p[var.as_str()] < *val => {
                        wf = res;
                        continue 'eval;
                    }
                    '>' if p[var.as_str()] > *val => {
                        wf = res;
                        continue 'eval;
                    }
                    '.' => {
                        wf = res;
                        continue 'eval;
                    }
                    _ => (),
                }
            }
        }
        if wf == "A" {
            sum += p.values().sum::<i64>()
        }
    }

    sum
}

pub fn part2(input: &str) -> i64 {
    let (rules_str, _) = split1(input, "\n\n");

    let rules = parse_rules(rules_str);

    let mut metapart = DefaultHashMap::new((1, 4000));
    metapart.insert("x".to_string(), (1, 4000));
    metapart.insert("m".to_string(), (1, 4000));
    metapart.insert("a".to_string(), (1, 4000));
    metapart.insert("s".to_string(), (1, 4000));

    fn recurse(
        mut mp: DefaultHashMap<String, (i64, i64)>,
        wf: &str,
        rules: &HashMap<String, Vec<(String, char, i64, String)>>,
    ) -> i64 {
        if wf == "A" {
            mp.values().map(|r| r.1 - r.0 + 1).product()
        } else if wf == "R" {
            0
        } else {
            let mut r = 0;
            for (var, op, val, res) in &rules[wf] {
                match op {
                    '>' => {
                        if mp[var].1 > *val {
                            let mut nmp = mp.clone();
                            nmp[var.clone()].0 = nmp[var].0.max(val + 1);
                            r += recurse(nmp, res, rules);
                            mp[var.clone()].1 = mp[var].1.min(*val);
                        }
                    }
                    '<' => {
                        if mp[var].0 < *val {
                            let mut nmp = mp.clone();
                            nmp[var.clone()].1 = nmp[var].1.min(val - 1);
                            r += recurse(nmp, res, rules);
                            mp[var.clone()].0 = mp[var].0.max(*val);
                        }
                    }
                    '.' => {
                        r += recurse(mp.clone(), res, rules);
                    }
                    _ => (),
                }
            }
            r
        }
    }

    recurse(metapart, "in", &rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 19114);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 342650);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 167409079868000);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 130303473508222);
    }
}
