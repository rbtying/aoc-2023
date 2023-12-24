use crate::prelude::*;

pub trait InfallibleFromStr<'a> {
    fn from_str_infallible(s: &'a str) -> Self;
}

macro_rules! impl_from_str_infallible {
    ( $($t:ty),* ) => {
        $( impl<'a> InfallibleFromStr<'a> for $t
        {
            fn from_str_infallible(s: &str) -> Self {
                <$t>::from_str(s.trim()).expect("Failed to parse string")
            }
        }) *
        }
}

impl_from_str_infallible!(bool);
impl_from_str_infallible!(i32);
impl_from_str_infallible!(i64);
impl_from_str_infallible!(u32);
impl_from_str_infallible!(u64);
impl_from_str_infallible!(isize);
impl_from_str_infallible!(usize);
impl_from_str_infallible!(f64);
impl_from_str_infallible!(String);

impl<'a> InfallibleFromStr<'a> for &'a str {
    fn from_str_infallible(s: &'a str) -> Self {
        s
    }
}

pub fn parse1<'a, A>(s1: &'a str) -> A
where
    A: InfallibleFromStr<'a>,
{
    A::from_str_infallible(s1)
}

pub fn parse_split_once<'a, 'b, 's: 'a + 'b, A, B>(s1: &'s str, delim: &str) -> (A, B)
where
    A: InfallibleFromStr<'a>,
    B: InfallibleFromStr<'b>,
{
    let (s1, s2) = s1.split_once(delim).unwrap();
    (A::from_str_infallible(s1), B::from_str_infallible(s2))
}

pub fn parse_left<'a, 's: 'a, A>(s1: &'s str, delim: &str) -> A
where
    A: InfallibleFromStr<'a>,
{
    let (s1, _) = s1.split_once(delim).unwrap();
    A::from_str_infallible(s1)
}

pub fn parse_right<'a, 's: 'a, A>(s1: &'s str, delim: &str) -> A
where
    A: InfallibleFromStr<'a>,
{
    let (_, s1) = s1.split_once(delim).unwrap();
    A::from_str_infallible(s1)
}

pub fn parse2<'a, 'b, 's: 'a + 'b, A, B>(mut s: impl Iterator<Item = &'s str>) -> (A, B)
where
    A: InfallibleFromStr<'a>,
    B: InfallibleFromStr<'b>,
{
    (parse1(s.next().unwrap()), parse1(s.next().unwrap()))
}

pub fn parse3<'a, 'b, 'c, 's: 'a + 'b + 'c, A, B, C>(
    mut s: impl Iterator<Item = &'s str>,
) -> (A, B, C)
where
    A: InfallibleFromStr<'a>,
    B: InfallibleFromStr<'b>,
    C: InfallibleFromStr<'c>,
{
    (
        parse1(s.next().unwrap()),
        parse1(s.next().unwrap()),
        parse1(s.next().unwrap()),
    )
}

pub fn parsen<'a, 's: 'a, A, I: Iterator<Item = &'s str> + 's>(s: I) -> impl Iterator<Item = A> + 's
where
    A: InfallibleFromStr<'a>,
{
    s.map(|x| parse1(x))
}

pub fn split1<'l>(s: &'l str, delim: &str) -> (&'l str, &'l str) {
    s.split_once(delim).unwrap()
}

pub fn drop_prefix<'l>(s: &'l str, prefix: &str) -> &'l str {
    split1(s, prefix).1
}

pub fn parse_ints(input: &str) -> Vec<i64> {
    let regex = Regex::new(r"-?\d+").unwrap();
    regex.find_iter(input).map(|x| parse1(x.as_str())).collect()
}

pub fn parse_floats(input: &str) -> Vec<f64> {
    let regex = Regex::new(r"-?\d+").unwrap();
    regex.find_iter(input).map(|x| parse1(x.as_str())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p3() {
        let s = "Game 1: 1 green, 2 red, 3 blue; 1 blue, 3 red, 4 green";
        let s = drop_prefix(s, "Game ");
        let (num, s) = split1(s, ": ");
        let num: u32 = parse1(num);

        let mut m: DefaultHashMap<&str, u32> = DefaultHashMap::default();

        for view in s.split("; ") {
            for x in view.split(", ") {
                let (n, color): (u32, &str) = parse2(x.split(' '));
                let c = m[color];
                m.insert(color, c + n);
            }
        }

        assert_eq!(num, 1);
        assert_eq!(m["green"], 5);
    }
}
