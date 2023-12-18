pub fn gcd(mut u: i64, mut v: i64) -> i64 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        v -= u;

        if v == 0 {
            break;
        }
    }

    u << shift
}

pub fn lcm(u: i64, v: i64) -> i64 {
    u * (v / gcd(u, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1000, 15), 5);
        assert_eq!(gcd(1000, 8000), 1000);
        assert_eq!(gcd(37, 8000), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(1000, 15), 3000);
        assert_eq!(lcm(1000, 8000), 8000);
        assert_eq!(lcm(37, 8000), 37 * 8000);
    }
}
