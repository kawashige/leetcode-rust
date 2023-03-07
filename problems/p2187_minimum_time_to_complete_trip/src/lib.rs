pub struct Solution {}

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut ng = 0;
        let mut ok = *time.iter().min().unwrap() as i64 * total_trips as i64;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if total_trips as i64 <= time.iter().map(|t| mid / *t as i64).sum::<i64>() {
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
    fn test_2187() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
        assert_eq!(Solution::minimum_time(vec![2], 1), 2);
    }
}
