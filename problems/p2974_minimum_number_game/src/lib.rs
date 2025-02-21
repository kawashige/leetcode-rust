pub struct Solution {}

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(2) {
            nums.swap(i, i + 1);
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2974() {
        assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
        assert_eq!(Solution::number_game(vec![2, 5]), vec![5, 2]);
    }
}
