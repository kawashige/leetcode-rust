pub struct Solution {}

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        ((n as i64 + 1) / 2) * (m as i64 / 2) + (n as i64 / 2) * ((m as i64 + 1) / 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3021() {
        assert_eq!(Solution::flower_game(3, 2), 3);
        assert_eq!(Solution::flower_game(1, 1), 0);
    }
}
