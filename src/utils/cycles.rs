use core::fmt;
use std::fmt::Debug;

use crate::prelude::*;

/// Note: starts with the `initial_state`, so if we need X iterations to have
/// passed we should index to X+1.
pub struct CycleInfo<S, K> {
    pub history: Vec<K>,
    pub offset: i64,
    pub cycle_len: i64,
    pub final_state: S,
}

impl<S, K> Index<i64> for CycleInfo<S, K> {
    type Output = K;

    fn index(&self, index: i64) -> &Self::Output {
        assert!(index >= 1);
        if (index as usize) < self.history.len() {
            &self.history[index as usize]
        } else {
            let offset2 = (index - self.offset) % self.cycle_len;
            &self.history[((self.offset + offset2) - 1) as usize]
        }
    }
}

impl<S, K: Debug> std::fmt::Debug for CycleInfo<S, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CycleInfo")
            .field("offset", &self.offset)
            .field("cycle_len", &self.cycle_len)
            .field("history", &self.history)
            .finish()
    }
}

/// Finds a cycle of the shape `offset` initial steps, then when `found_cycle`
/// is true for the first time, it will be true every `cycle_len` values.
pub fn find_cycle_generic<S, K>(
    initial_state: S,
    next: impl Fn(&mut S),
    extract_key: impl Fn(&S) -> K,
    found_cycle: impl Fn(&K, &K) -> bool,
) -> CycleInfo<S, K> {
    let mut history = vec![extract_key(&initial_state)];
    let mut s = initial_state;

    loop {
        next(&mut s);
        let k = extract_key(&s);

        if let Some(offset) = history.iter().position(|kk| found_cycle(&k, kk)) {
            let offset = offset as i64;
            let cycle_len = history.len() as i64 - offset;
            history.push(k);

            return CycleInfo {
                history,
                offset,
                cycle_len,
                final_state: s,
            };
        }

        history.push(k);
    }
}

/// Finds a cycle of the shape `offset` initial steps, then when the key is
/// equal to an earlier key for the first time, it will reoccur every
/// `cycle_len` values.
pub fn find_cycle_equals<S, K: Eq + Hash>(
    initial_state: S,
    next: impl Fn(&mut S),
    extract_key: impl Fn(&S) -> K,
) -> CycleInfo<S, K> {
    let mut history = HashMap::new();
    history.insert(extract_key(&initial_state), 0);
    let mut s = initial_state;

    for idx in 1.. {
        next(&mut s);
        let k = extract_key(&s);

        if let Some(offset) = history.get(&k) {
            let offset = *offset;
            let cycle_len = history.len() as i64 - offset;
            let mut v = history.into_iter().collect::<Vec<(K, i64)>>();
            v.sort_by_key(|vv| vv.1);
            let history = v
                .into_iter()
                .map(|vv| vv.0)
                .chain(Some(k))
                .collect::<Vec<K>>();

            return CycleInfo {
                history,
                offset,
                cycle_len,
                final_state: s,
            };
        }

        history.insert(k, idx);
    }
    unreachable!()
}
