pub struct Solution {}

impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        (if k < n { k as i64 * (n - k) as i64 } else { 0 })
            + if k < m { k as i64 * (m - k) as i64 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3560() {
        assert_eq!(Solution::min_cutting_cost(6, 5, 5), 5);
        assert_eq!(Solution::min_cutting_cost(4, 4, 6), 0);
    }
}
