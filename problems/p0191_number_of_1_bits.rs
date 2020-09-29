pub struct Solution {}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        (0..32).fold(0, |count, i| count + ((n >> i) & 1) as i32)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0191() {
        assert_eq!(
            3,
            Solution::hammingWeight(0b00000000000000000000000000001011u32)
        );
    }
}
