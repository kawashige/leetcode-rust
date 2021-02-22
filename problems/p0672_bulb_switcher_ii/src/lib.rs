use std::unreachable;

pub struct Solution {}

impl Solution {
    pub fn flip_lights(n: i32, m: i32) -> i32 {
        match (n, m) {
            (0, _) => 0,
            (_, 0) => 1,
            (1, 1) => 2,
            (2, 1) => 3,
            (_, 1) => 4,
            (1, 2) => 2,
            (2, 2) => 4,
            (_, 2) => 7,
            (1, _) => 2,
            (2, _) => 4,
            (_, _) => 8,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0672() {
        assert_eq!(Solution::flip_lights(1, 1), 2);
        assert_eq!(Solution::flip_lights(2, 1), 3);
        assert_eq!(Solution::flip_lights(3, 1), 4);
    }
}
