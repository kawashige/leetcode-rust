pub struct Solution {}

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut ways = 0;

        for pen in 0..=total / cost1 {
            ways += (total as i64 - pen as i64 * cost1 as i64) / cost2 as i64 + 1;
        }

        ways
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2240() {
        assert_eq!(Solution::ways_to_buy_pens_pencils(20, 10, 5), 9);
        assert_eq!(Solution::ways_to_buy_pens_pencils(5, 10, 10), 1);
    }
}
