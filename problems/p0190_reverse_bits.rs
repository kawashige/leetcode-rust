pub struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        (0..32).fold(0, |acc, i| {
            if 0 < x & 1 << i {
                acc | (1 << (31 - i))
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0190() {
        assert_eq!(
            964176192,
            Solution::reverse_bits(0b00000010100101000001111010011100)
        );
        assert_eq!(
            3221225471,
            Solution::reverse_bits(0b11111111111111111111111111111101)
        );
    }
}
