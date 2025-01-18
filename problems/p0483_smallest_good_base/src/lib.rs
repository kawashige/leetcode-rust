pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: usize, l: usize, n: usize) -> bool {
        let mut x: usize = 1;
        let mut sum: usize = 1;

        for _ in 0..l - 1 {
            x = x.saturating_mul(mid);
            sum = sum.saturating_add(x);
            if n < x || n < sum {
                return true;
            }
        }
        n <= sum
    }

    pub fn check(mid: usize, l: usize, n: usize) -> bool {
        let mut x = 1;
        let mut sum = 1;

        for _ in 0..l - 1 {
            x *= mid;
            sum += x;
            if n < x || n < sum {
                return false;
            }
        }
        n == sum
    }

    pub fn smallest_good_base(n: String) -> String {
        let n_d = n.parse::<usize>().unwrap();
        let mut x = 1;
        let mut sum = 1;
        let mut l = 1;
        while sum < n_d {
            x *= 2;
            l += 1;
            sum += x;
        }

        for i in (2..=l).rev() {
            let mut ng = 1;
            let mut ok = n_d;

            while ng + 1 < ok {
                let mid = (ok + ng) / 2;
                if Self::is_ok(mid, i, n_d) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            if Self::check(ok, i, n_d) {
                return ok.to_string();
            }
        }

        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0483() {
        assert_eq!(
            Solution::smallest_good_base("13".to_string()),
            "3".to_string()
        );
        assert_eq!(
            Solution::smallest_good_base("4681".to_string()),
            "8".to_string()
        );
        assert_eq!(
            Solution::smallest_good_base("1000000000000000000".to_string()),
            "999999999999999999".to_string()
        );
    }
}
