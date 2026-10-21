pub struct Solution {}

impl Solution {
    pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
        let count = nums.iter().filter(|n| n == &&0).count();
        nums.into_iter()
            .rev()
            .take(count)
            .filter(|n| n != &0)
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3936() {
        assert_eq!(Solution::minimum_swaps(vec![0, 1, 0, 3, 12]), 2);
        assert_eq!(Solution::minimum_swaps(vec![0, 1, 0, 2]), 1);
        assert_eq!(Solution::minimum_swaps(vec![1, 2, 0]), 0);
    }
}

fn main() {}
