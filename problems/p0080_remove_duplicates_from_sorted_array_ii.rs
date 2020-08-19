pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len() as i32;
        if n > 0 {
            for i in (1..(nums.len() - 1)).rev() {
                if nums[i] == nums[i + 1] && nums[i] == nums[i - 1] {
                    nums.remove(i);
                    n -= 1;
                }
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0080() {
        let mut input = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(5, Solution::remove_duplicates(&mut input));
        assert_eq!(vec![1, 1, 2, 2, 3], input);

        let mut input2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(7, Solution::remove_duplicates(&mut input2));
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], input2);

        let mut input3 = vec![0];
        assert_eq!(1, Solution::remove_duplicates(&mut input3));
        assert_eq!(vec![0], input3);

        let mut input4 = vec![0, 0];
        assert_eq!(2, Solution::remove_duplicates(&mut input4));
        assert_eq!(vec![0, 0], input4);

        let mut input5 = vec![];
        assert_eq!(0, Solution::remove_duplicates(&mut input5));
        assert_eq!(Vec::new() as Vec<i32>, input5);
    }
}
