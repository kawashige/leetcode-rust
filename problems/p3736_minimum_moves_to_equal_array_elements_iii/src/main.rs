pub struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        nums.into_iter().map(|n| max - n).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3736() {
        assert_eq!(Solution::min_moves(vec![2, 1, 3]), 3);
        assert_eq!(Solution::min_moves(vec![4, 4, 5]), 2);
    }
}

fn main() {}
