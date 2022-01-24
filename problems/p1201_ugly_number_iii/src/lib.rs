pub struct Solution {}

impl Solution {
    pub fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }

    pub fn is_ok(x: i64, n: i64, a: i64, b: i64, c: i64) -> bool {
        x / a + x / b + x / c - x / Self::lcm(a, b) - x / Self::lcm(a, c) - x / Self::lcm(b, c)
            + x / Self::lcm(Self::lcm(a, b), c)
            >= n as i64
    }

    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let mut v = [a as i64, b as i64, c as i64];
        v.sort_unstable();

        let mut ng = 0;
        let mut ok = 2 * 1_000_000_000;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, n as i64, v[0], v[1], v[2]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1201() {
        assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
        assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
        assert_eq!(Solution::nth_ugly_number(5, 2, 11, 13), 10);
    }
}
