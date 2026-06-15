pub struct Solution {}

impl Solution {
    pub fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }

    pub fn is_ok(mid: i64, d: &[i64], r: &[i64]) -> bool {
        let r0 = mid / r[0];
        let r1 = mid / r[1];

        let c0 = mid - r0;
        let c1 = mid - r1;
        let shared = mid - r0 - r1 + mid / Self::lcm(r[0], r[1]);

        (d[0] - (c0 - shared)).max(0) + (d[1] - (c1 - shared)).max(0) <= shared
    }

    pub fn minimum_time(d: Vec<i32>, r: Vec<i32>) -> i64 {
        let d = d.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let r = r.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut ng = 0;
        let mut ok = *d.iter().max().unwrap() * 4;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, &d, &r) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3733() {
        assert_eq!(Solution::minimum_time(vec![3, 1], vec![2, 3]), 5);
        assert_eq!(Solution::minimum_time(vec![1, 3], vec![2, 2]), 7);
        assert_eq!(Solution::minimum_time(vec![2, 1], vec![3, 4]), 3);
    }
}

fn main() {}
