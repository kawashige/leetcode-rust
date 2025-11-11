pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, h: i64, worker_times: &[i32]) -> bool {
        let mut sum = 0;

        for i in 0..worker_times.len() {
            let mut ok = 0;
            let mut ng = mid as i128 + 1;
            while ok + 1 < ng {
                let m = (ok + ng) / 2;
                if worker_times[i] as i128 * m * (m + 1) / 2 <= mid as i128 {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            sum += ok;
        }

        h as i128 <= sum
    }

    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let h = mountain_height as i64;
        let mut ng = 0;
        let mut ok = worker_times[0] as i64 * h * (h + 1) / 2;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, h, &worker_times) {
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
    fn test_3296() {
        assert_eq!(
            Solution::min_number_of_seconds(100000, vec![1000000]),
            10520477
        );
        assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
        assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
        assert_eq!(Solution::min_number_of_seconds(5, vec![1]), 15);
    }
}
