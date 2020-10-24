pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == 0 {
                let mut j = i;
                while j + 1 < nums.len() && nums[j] == 0 {
                    j += 1;
                }
                if nums[j] == 0 {
                    break;
                }
                nums.swap(i, j);
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0283() {
        let mut input = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut input);
        assert_eq!(vec![1, 3, 12, 0, 0], input);

        let mut input = vec![];
        Solution::move_zeroes(&mut input);
        assert_eq!(vec![] as Vec<i32>, input);

        let mut input = vec![1, 2, 3];
        Solution::move_zeroes(&mut input);
        assert_eq!(vec![1, 2, 3], input);

        let mut input = vec![0, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(vec![0, 0, 0], input);
    }
}
