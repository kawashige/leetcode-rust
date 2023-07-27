pub struct Solution {}

impl Solution {
    pub fn is_ok(m: i64, n: i64, batteries: &[i64]) -> bool {
        let mut c = 0;
        let mut r = 0;
        for i in 0..batteries.len() {
            c += batteries[i].min(m) / m;
            r += batteries[i].min(m) % m;
        }
        n <= c + r / m
    }

    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut batteries = batteries.into_iter().map(|b| b as i64).collect::<Vec<_>>();
        batteries.sort_unstable_by(|a, b| b.cmp(&a));

        let n = n as i64;
        let mut ok = 0;
        let mut ng = batteries.iter().sum::<i64>() / n + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, n, &batteries) {
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
    fn test_2141() {
        assert_eq!(Solution::max_run_time(3, vec![10, 10, 3, 5]), 8);
        assert_eq!(Solution::max_run_time(2, vec![3, 3, 3]), 4);
        assert_eq!(Solution::max_run_time(2, vec![1, 1, 1, 1]), 2);
    }
}
