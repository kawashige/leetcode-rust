pub struct Solution {}

impl Solution {
    pub fn ok(bloom_day: &[i32], day: i32, k: i32, m: i32) -> bool {
        let mut bouquets = 0;
        let mut adjacent = 0;

        for x in bloom_day {
            if x <= &day {
                adjacent += 1;
                if adjacent == k {
                    bouquets += 1;
                    adjacent = 0;
                }
            } else {
                adjacent = 0;
            }
        }

        m <= bouquets
    }

    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if (bloom_day.len() as i64) < m as i64 * k as i64 {
            return -1;
        }

        let mut ok = *bloom_day.iter().max().unwrap();
        let mut ng = 0;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::ok(&bloom_day, mid, k, m) {
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
    fn test_1482() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }
}
