pub struct Solution {}

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (0..32).filter(|i| start & 1 << i != goal & 1 << i).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2220() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
