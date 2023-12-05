use std::ops::RangeInclusive;

/// Converts from (base, len) to base..=base+len
pub fn interval_base_and_len_to_range((base, len): (isize, isize)) -> RangeInclusive<isize> {
    base..=base + len
}

/// Converts from start..=end to (start, end-start)
pub fn range_to_base_and_len(interval: RangeInclusive<isize>) -> (isize, isize) {
    (*interval.start(), interval.end() - interval.start())
}

/// Returns true if the intervals overlaps, including if they share an endpoint.
pub fn overlaps(a: RangeInclusive<isize>, b: RangeInclusive<isize>) -> bool {
    let e = a.start().max(b.start());
    let f = a.end().min(b.end());
    e <= f
}

/// Returns true if the intervals overlap with a nontrivial interval.
pub fn overlaps_nontrivial(a: RangeInclusive<isize>, b: RangeInclusive<isize>) -> bool {
    let e = a.start().max(b.start());
    let f = a.end().min(b.end());
    e < f
}

pub enum OverlapResult {
    NonOverlapping {
        a: RangeInclusive<isize>,
        b: RangeInclusive<isize>,
    },
    Adjacent {
        a: RangeInclusive<isize>,
        b: RangeInclusive<isize>,
    },
    Overlapping {
        overlap: RangeInclusive<isize>,
        a: Vec<RangeInclusive<isize>>,
        b: Vec<RangeInclusive<isize>>,
    },
}

#[allow(clippy::comparison_chain)]
pub fn compute_overlaps(a: RangeInclusive<isize>, b: RangeInclusive<isize>) -> OverlapResult {
    let e = a.start().max(b.start());
    let f = a.end().min(b.end());

    if e < f {
        let mut a_nonoverlapping = vec![];

        if a.start() < e {
            a_nonoverlapping.push(*a.start()..=*e);
        }
        if a.end() > f {
            a_nonoverlapping.push(*f..=*a.end());
        }

        let mut b_nonoverlapping = vec![];
        if b.start() < e {
            b_nonoverlapping.push(*b.start()..=*e);
        }
        if b.end() > f {
            b_nonoverlapping.push(*f..=*b.end());
        }

        OverlapResult::Overlapping {
            overlap: *e..=*f,
            a: a_nonoverlapping,
            b: b_nonoverlapping,
        }
    } else if e == f {
        OverlapResult::Adjacent { a, b }
    } else {
        OverlapResult::NonOverlapping { a, b }
    }
}
