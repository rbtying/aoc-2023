use std::{fmt::Debug, ops::RangeInclusive};

/// Converts from (base, len) to base..=base+len
pub fn interval_base_and_len_to_range((base, len): (i64, i64)) -> RangeInclusive<i64> {
    base..=base + len
}

/// Converts from start..=end to (start, end-start)
pub fn range_to_base_and_len(interval: RangeInclusive<i64>) -> (i64, i64) {
    (*interval.start(), interval.end() - interval.start())
}

/// Returns true if the intervals overlaps, including if they share an endpoint.
pub fn overlaps(a: RangeInclusive<i64>, b: RangeInclusive<i64>) -> bool {
    let e = a.start().max(b.start());
    let f = a.end().min(b.end());
    e <= f
}

/// Returns true if the intervals overlap with a nontrivial interval.
pub fn overlaps_nontrivial(a: RangeInclusive<i64>, b: RangeInclusive<i64>) -> bool {
    let e = a.start().max(b.start());
    let f = a.end().min(b.end());
    e < f
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlapResult {
    NonOverlapping {
        a: RangeInclusive<i64>,
        b: RangeInclusive<i64>,
    },
    Overlapping {
        overlap: RangeInclusive<i64>,
        a: Vec<RangeInclusive<i64>>,
        b: Vec<RangeInclusive<i64>>,
    },
}

#[allow(clippy::comparison_chain)]
pub fn compute_overlaps(a: RangeInclusive<i64>, b: RangeInclusive<i64>) -> OverlapResult {
    let e = a.start().max(b.start());
    let f = a.end().min(b.end());

    if e <= f {
        let mut a_nonoverlapping = vec![];

        if *a.start() < e - 1 {
            a_nonoverlapping.push(*a.start()..=e - 1);
        }
        if *a.end() > f + 1 {
            a_nonoverlapping.push(f + 1..=*a.end());
        }

        let mut b_nonoverlapping = vec![];
        if *b.start() < e - 1 {
            b_nonoverlapping.push(*b.start()..=e - 1);
        }
        if *b.end() > f + 1 {
            b_nonoverlapping.push(f + 1..=*b.end());
        }

        OverlapResult::Overlapping {
            overlap: *e..=*f,
            a: a_nonoverlapping,
            b: b_nonoverlapping,
        }
    } else {
        OverlapResult::NonOverlapping { a, b }
    }
}

pub fn merge_overlapping_intervals<T: Clone + Debug>(
    mut intervals: Vec<(RangeInclusive<i64>, T)>,
    reduce: impl Fn(T, T) -> T,
) -> Vec<(RangeInclusive<i64>, T)> {
    intervals.sort_by_key(|(r, _)| -r.start());

    let mut new_intervals = Vec::with_capacity(intervals.len());

    while let Some((next, next_v)) = intervals.pop() {
        match new_intervals.last_mut() {
            None => new_intervals.push((next, next_v)),
            // Next interval starts after the start of the current interval
            Some((prev, prev_v)) if next.start() >= prev.start() => {
                match compute_overlaps(prev.clone(), next.clone()) {
                    OverlapResult::NonOverlapping { .. } => new_intervals.push((next, next_v)),
                    OverlapResult::Overlapping { overlap, a, b } => {
                        let prev_v = prev_v.clone();
                        new_intervals.pop();
                        let mut to_add = vec![];

                        to_add.extend(a.into_iter().map(|r| (r, prev_v.clone())));
                        to_add.push((overlap, reduce(prev_v.clone(), next_v.clone())));
                        to_add.extend(b.into_iter().map(|r| (r, next_v.clone())));

                        to_add.sort_by_key(|(r, _)| *r.start());
                        new_intervals.extend(to_add);
                    }
                }
            }
            // Next interval starts *before* the start of the current interval,
            // so we need to pop items until that's no longer true.
            Some(_) => {
                let start_value = *next.start();

                while let Some(p) = new_intervals.last() {
                    if *p.0.start() < start_value {
                        break;
                    } else {
                        intervals.extend(new_intervals.pop());
                    }
                }
                intervals.push((next, next_v));
            }
        }
    }

    new_intervals
}

pub fn collapse_adjacent_intervals<T: Eq>(
    iter: impl IntoIterator<Item = (RangeInclusive<i64>, T)>,
) -> Vec<(RangeInclusive<i64>, T)> {
    let mut ret: Vec<(RangeInclusive<i64>, T)> = vec![];

    for (new_range, new_v) in iter {
        match ret.last_mut() {
            Some((range, v)) if *v == new_v => {
                *range = *range.start().min(new_range.start())..=*range.end().max(new_range.end())
            }
            None | Some(_) => ret.push((new_range, new_v)),
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_overlaps() {
        let intervals = vec![(0..=5, 0), (3..=8, 1), (20..=25, 2)];
        assert_eq!(
            merge_overlapping_intervals(intervals.clone(), |a, b| a.min(b)),
            vec![(0..=2, 0), (3..=5, 0), (6..=8, 1), (20..=25, 2)]
        );
        assert_eq!(
            collapse_adjacent_intervals(merge_overlapping_intervals(intervals, |a, b| a.min(b))),
            vec![(0..=5, 0), (6..=8, 1), (20..=25, 2)]
        );

        let intervals = vec![(0..=5, 0), (0..=8, 1), (-10..=3, 2)];
        assert_eq!(
            merge_overlapping_intervals(intervals, |a, b| a.min(b)),
            vec![(-10..=-1, 2), (0..=3, 0), (4..=5, 0), (6..=8, 1),]
        );
    }

    #[test]
    fn test_find_overlaps() {
        assert_eq!(
            compute_overlaps(0..=5, 3..=8),
            OverlapResult::Overlapping {
                overlap: 3..=5,
                a: vec![0..=2],
                b: vec![6..=8],
            }
        );
        assert_eq!(
            compute_overlaps(0..=5, 6..=8),
            OverlapResult::NonOverlapping { a: 0..=5, b: 6..=8 }
        );

        assert_eq!(
            compute_overlaps(0..=8, 3..=5),
            OverlapResult::Overlapping {
                overlap: 3..=5,
                a: vec![0..=2, 6..=8],
                b: vec![],
            }
        );
    }
}
