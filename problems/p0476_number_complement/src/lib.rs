pub struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        (0..(32 - num.leading_zeros())).fold(
            0,
            |acc, i| {
                if 0 < num & 1 << i {
                    acc
                } else {
                    acc | 1 << i
                }
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0476() {
        assert_eq!(2, Solution::find_complement(5));
        assert_eq!(0, Solution::find_complement(1));
        assert_eq!(0, Solution::find_complement(i32::MAX));
    }
}
