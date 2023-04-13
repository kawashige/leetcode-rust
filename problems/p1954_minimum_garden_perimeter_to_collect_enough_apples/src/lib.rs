pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, needed_apples: i64) -> bool {
        let apples = mid
            .saturating_pow(3)
            .saturating_mul(4)
            .saturating_add(mid.saturating_pow(2).saturating_mul(6))
            .saturating_add(mid.saturating_mul(2));

        needed_apples <= apples
    }

    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut ng = 0;
        let mut ok = needed_apples;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, needed_apples) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok * 8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1954() {
        assert_eq!(Solution::minimum_perimeter(1), 8);
        assert_eq!(Solution::minimum_perimeter(13), 16);
        assert_eq!(Solution::minimum_perimeter(1000000000), 5040);
        assert_eq!(Solution::minimum_perimeter(100000000000000), 233920);
    }
}
