pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

mod utils;
pub mod prelude {
    pub use std::cmp::*;
    pub use std::collections::*;
    pub use std::fs;
    pub use std::io::{BufRead, BufReader, BufWriter, Read, Write};
    pub use std::iter::IntoIterator;
    pub use std::ops::*;
    pub use std::str::FromStr;

    pub use defaultmap::*;
    pub use regex::{Regex, RegexBuilder, RegexSet};

    pub use crate::utils::*;

    pub fn read_day_input(module_path: &str) -> String {
        let path = format!(
            "puzzle/{}/input",
            split1(split1(module_path, "::").1, "::").0
        );
        std::fs::read_to_string(path).unwrap()
    }
}
