pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, dist: &[i32], hour: i64) -> bool {
        let sum = dist[..dist.len() - 1]
            .iter()
            .map(|d| (*d as i64 + mid - 1) / mid)
            .sum::<i64>();
        if hour <= sum * 100 {
            return false;
        }
        *dist.last().unwrap() as i64 * 100 <= mid * (hour - sum * 100)
    }

    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if hour <= dist.len() as f64 - 1.0 {
            return -1;
        }
        let hour = format!("{:.2}", hour)
            .replace('.', "")
            .parse::<i64>()
            .unwrap();

        let mut ok = 10_000_000;
        let mut ng = 0;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &dist, hour) {
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
    fn test_1870() {
        assert_eq!(Solution::min_speed_on_time(vec![1, 2, 3], 2.03), 100);
        assert_eq!(Solution::min_speed_on_time(vec![69], 4.6), 15);
        assert_eq!(
            Solution::min_speed_on_time(vec![1, 1, 100000], 2.01),
            10000000
        );
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 6.0), 1);
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 2.7), 3);
        assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 1.9), -1);
    }
}
