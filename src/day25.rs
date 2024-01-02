use crate::prelude::*;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> i64 {
    let mut g = DiGraphMap::default();

    for line in input.lines() {
        let (from, to) = split1(line, ": ");
        for t in to.split(' ') {
            g.add_edge(from, t, 1);
            g.add_edge(t, from, 1);
        }
    }

    let nodes = g.nodes().collect::<Vec<_>>();

    for (idx, n) in nodes.iter().enumerate() {
        for nn in nodes.iter().skip(idx + 1) {
            assert_ne!(n, nn);
            let res = edmonds_karp(&g, n, nn);
            if res.min_cut.len() == 3 {
                return (res.source_partition.len() * res.sink_partition.len()) as i64;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 54);
    }
}
