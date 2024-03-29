pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

mod utils;
pub mod prelude {
    pub use std::borrow::*;
    pub use std::cmp::*;
    pub use std::collections::*;
    pub use std::fs;
    pub use std::hash::*;
    pub use std::io::{BufRead, BufReader, BufWriter, Read, Write};
    pub use std::iter::{FromIterator, IntoIterator};
    pub use std::ops::*;
    pub use std::str::FromStr;

    pub use ::num::bigint::*;
    pub use ::num::rational::*;
    pub use aoc_runner_derive::*;
    pub use euclid::*;
    pub use fnv::*;
    pub use petgraph::prelude::*;
    pub use regex::{Regex, RegexBuilder, RegexSet};

    pub type HashMap<K, V> = FnvHashMap<K, V>;
    pub type HashSet<K> = FnvHashSet<K>;

    pub use crate::utils::*;
}

aoc_runner_derive::aoc_lib! { year = 2023 }
