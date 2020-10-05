pub struct Solution {}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        match n {
            0 => 1,
            _ => n ^ (0..(32 - n.leading_zeros())).fold(0, |acc, i| acc | 1 << i),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day5() {
        assert_eq!(2, Solution::bitwise_complement(5));
        assert_eq!(0, Solution::bitwise_complement(7));
        assert_eq!(5, Solution::bitwise_complement(10));
    }
}
