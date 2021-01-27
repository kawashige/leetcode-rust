pub struct Solution {}

impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..nums.len() {
            if nums[i] == -1 {
                continue;
            }
            let mut l = 1;
            let mut j = nums[i] as usize;
            nums[i] = -1;
            while i != j {
                let tmp = j;
                j = nums[j] as usize;
                nums[tmp] = -1;
                l += 1;
            }
            max = std::cmp::max(max, l);
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0565() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
        assert_eq!(Solution::array_nesting(vec![0]), 1);
    }
}
