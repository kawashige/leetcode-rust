pub struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x as u32 ^ y as u32).count_ones() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0461() {
        assert_eq!(2, Solution::hamming_distance(1, 4));
    }
}
