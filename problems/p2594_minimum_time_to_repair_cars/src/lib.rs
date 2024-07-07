pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, cars: i64, ranks: &[i32]) -> bool {
        cars <= ranks
            .iter()
            .fold(0, |acc, r| acc + ((mid / *r as i64) as f64).sqrt() as i64)
    }

    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut ranks = ranks;
        ranks.sort_unstable();

        let mut ng = 0;
        let mut ok = ranks[0] as i64 * cars as i64 * cars as i64;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, cars as i64, &ranks) {
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
    fn test_2594() {
        assert_eq!(Solution::repair_cars(vec![4, 2, 3, 1], 10), 16);
        assert_eq!(Solution::repair_cars(vec![5, 1, 8], 6), 16);
    }
}
