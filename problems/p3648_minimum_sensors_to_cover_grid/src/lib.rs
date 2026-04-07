pub struct Solution {}

impl Solution {
    pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        let s = 2 * k + 1;
        ((n + s - 1) / s) * ((m + s - 1) / s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3648() {
        assert_eq!(Solution::min_sensors(5, 5, 1), 4);
        assert_eq!(Solution::min_sensors(2, 2, 2), 1);
    }
}
