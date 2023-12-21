use crate::prelude::*;

#[derive(Debug)]
pub struct NewtonPolynomial {
    pub p: Vec<Rational64>,
    pub x: Vec<i64>,
}

impl NewtonPolynomial {
    pub fn eval(&self, x: i64) -> i64 {
        let mut y = Rational64::from_integer(0);
        for (i, p_i) in self.p.iter().enumerate() {
            let mut t = *p_i;
            for j in 0..i {
                t *= x - self.x[j];
            }
            y += t;
        }
        assert!(y.is_integer());
        y.to_integer()
    }
}

pub fn differences(x: &[i64]) -> Vec<i64> {
    x.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn polynomial_regression(x: &[i64], y: &[i64], n: usize) -> NewtonPolynomial {
    assert!(x.len() >= n + 1);
    assert_eq!(x.len(), y.len());

    let mut p = vec![Rational64::from_integer(y[0])];

    for i in 1..x.len() {
        let mut term = Rational64::from_integer(y[i]);
        for j in 0..i {
            term = (term - p[j]) / (x[i] - x[j])
        }

        p.push(term);
    }

    let n = NewtonPolynomial {
        p,
        x: x[..n + 1].to_vec(),
    };

    for (xx, yy) in x.iter().zip(y.iter()) {
        let v = n.eval(*xx);
        assert_eq!(
            v,
            *yy,
            "p({}) = {}, got {} | x={:?}, y={:?}, p={:?}, n={:?}",
            xx,
            yy,
            v,
            x,
            y,
            n.p,
            n.x.len(),
        );
    }

    n
}

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

    #[test]
    fn test_newton() {
        let x = vec![1, 2, 3, 4];
        let y = vec![2, 5, 10, 17];
        let p = polynomial_regression(&x, &y, 2);
        assert_eq!(p.eval(4), 17);
        assert_eq!(p.eval(0), 1);
        assert_eq!(p.eval(-1), 2);
        assert_eq!(p.eval(-5), 26);

        let x = vec![1, 2, 10, 20];
        let y = vec![1, 8, 1000, 8000];
        polynomial_regression(&x, &y, 3);
    }
}
