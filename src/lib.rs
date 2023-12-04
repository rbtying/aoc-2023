pub mod day01;
pub mod day02;
pub mod day03;

mod utils;
pub mod prelude {
    pub use std::cmp::*;
    pub use std::collections::*;
    pub use std::iter::IntoIterator;
    pub use std::str::FromStr;

    pub use defaultmap::*;
    pub use regex::{Regex, RegexBuilder, RegexSet};

    pub use crate::utils::*;
}
