pub struct Solution {}

impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|n| n % 2 == 0)
            .fold(0, |acc, n| acc | n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3688() {
        assert_eq!(
            Solution::even_number_bitwise_o_rs(vec![1, 2, 3, 4, 5, 6]),
            6
        );
        assert_eq!(Solution::even_number_bitwise_o_rs(vec![7, 9, 11]), 0);
        assert_eq!(Solution::even_number_bitwise_o_rs(vec![1, 8, 16]), 24);
    }
}

fn main() {}
