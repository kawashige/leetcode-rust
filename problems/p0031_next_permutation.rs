pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let l = nums.len();
        for i in 0..(l - 1) {
            let j = l - 1 - i;
            if nums[j - 1] < nums[j] {
                let mut k = l - 1;
                while j - 1 < k {
                    if nums[j - 1] < nums[k] {
                        let tmp = nums[j - 1];
                        nums[j - 1] = nums[k];
                        nums[k] = tmp;
                        break;
                    }
                    k -= 1;
                }
                nums[j..].sort();
                return;
            }
        }
        nums.sort();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0031() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 3, 2], nums);

        let mut nums2 = vec![3, 2, 1];
        Solution::next_permutation(&mut nums2);
        assert_eq!(vec![1, 2, 3], nums2);

        let mut nums2 = vec![1, 3, 2];
        Solution::next_permutation(&mut nums2);
        assert_eq!(vec![2, 1, 3], nums2);
    }
}
