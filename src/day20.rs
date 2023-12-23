use petgraph::graph::NodeIndex;

use crate::prelude::*;

#[derive(Debug)]
enum Module {
    Broadcaster,
    Flipflop(bool),
    Conjunction(DefaultHashMap<NodeIndex, bool>),
    Unknown,
}

fn build_graph(input: &str) -> (Graph<Module, ()>, HashMap<&'_ str, NodeIndex>) {
    let mut g = Graph::<Module, ()>::new();
    let mut name = HashMap::default();

    for line in input.lines() {
        let (module, recipients) = split1(line, " -> ");
        let recipients = recipients.split(", ");
        let state = match module.chars().next().unwrap() {
            'b' => Module::Broadcaster,
            '%' => Module::Flipflop(false),
            '&' => Module::Conjunction(DefaultHashMap::default()),
            _ => unreachable!(),
        };
        let n = module.trim_start_matches(|c| c == '%' || c == '&');

        let n_idx = *name.entry(n).or_insert_with(|| g.add_node(Module::Unknown));
        *g.node_weight_mut(n_idx).unwrap() = state;

        for recipient in recipients {
            let r_idx = *name
                .entry(recipient)
                .or_insert_with(|| g.add_node(Module::Unknown));
            g.add_edge(n_idx, r_idx, ());
        }
    }
    (g, name)
}

pub fn part1(input: &str) -> i64 {
    let (mut g, name) = build_graph(input);

    // Press the button once -> send a low pulse to broadcaster
    let mut num_low_pulses = 0;
    let mut num_high_pulses = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back((name["broadcaster"], name["broadcaster"], false));

        while let Some((from, node, pulse_val)) = pulses.pop_front() {
            if pulse_val {
                num_low_pulses += 1;
            } else {
                num_high_pulses += 1;
            }

            let incoming = g
                .edges_directed(node, Incoming)
                .map(|e| e.source())
                .collect::<Vec<_>>();

            let output = match g.node_weight_mut(node).unwrap() {
                Module::Broadcaster => Some(pulse_val),
                Module::Conjunction(ref mut received) => {
                    received[from] = pulse_val;

                    if incoming.iter().all(|i| received[i]) {
                        Some(false)
                    } else {
                        Some(true)
                    }
                }
                Module::Flipflop(_) if pulse_val => None,
                Module::Flipflop(ref mut x) => {
                    *x ^= true;
                    Some(*x)
                }
                Module::Unknown => None,
            };
            if let Some(pulse) = output {
                for recipient in g.edges_directed(node, Outgoing) {
                    pulses.push_back((node, recipient.target(), pulse));
                }
            }
        }
    }

    num_low_pulses * num_high_pulses
}

pub fn part2(input: &str) -> i64 {
    let (mut g, name) = build_graph(input);

    // find the important nodes -- the node immediately prior to the rx is a
    // conjunction, so it's all the inputs to the conjunction
    let mut important = HashSet::default();
    for src in g.edges_directed(name["rx"], Incoming).map(|e| e.source()) {
        let incoming = g
            .edges_directed(src, Incoming)
            .map(|e| e.source())
            .collect::<Vec<_>>();
        assert!(important.is_empty());
        important.extend(incoming);
    }

    let mut prev: HashMap<_, i64> = HashMap::default();
    let mut cyc = HashMap::default();

    for idx in 1.. {
        let mut pulses = VecDeque::new();
        pulses.push_back((name["broadcaster"], name["broadcaster"], false));

        while let Some((from, node, pulse_val)) = pulses.pop_front() {
            if !pulse_val {
                if prev.contains_key(&node) && !cyc.contains_key(&node) && important.contains(&node)
                {
                    cyc.insert(node, idx - prev[&node]);
                }

                prev.insert(node, idx);
            }

            if cyc.len() == important.len() {
                return cyc.values().copied().reduce(lcm).unwrap();
            }

            let incoming = g
                .edges_directed(node, Incoming)
                .map(|e| e.source())
                .collect::<Vec<_>>();

            let output = match g.node_weight_mut(node).unwrap() {
                Module::Broadcaster => Some(pulse_val),
                Module::Conjunction(ref mut received) => {
                    received[from] = pulse_val;

                    if incoming.iter().all(|i| received[i]) {
                        Some(false)
                    } else {
                        Some(true)
                    }
                }
                Module::Flipflop(_) if pulse_val => None,
                Module::Flipflop(ref mut x) => {
                    *x ^= true;
                    Some(*x)
                }
                Module::Unknown => None,
            };
            if let Some(pulse) = output {
                for recipient in g.edges_directed(node, Outgoing) {
                    pulses.push_back((node, recipient.target(), pulse));
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 32000000);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&read_day_input(std::module_path!())), 731517480);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&read_day_input(std::module_path!())), 244178746156661);
    }
}
