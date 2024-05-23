pub struct Solution {}

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut count = 0;

        for i in 1..2_usize.pow(nums.len() as u32) {
            let mut found = vec![false; 1001];
            let mut is_beautiful = true;
            for j in 0..=nums.len() {
                if i & 1 << j == 0 {
                    continue;
                }
                if k < nums[j] && found[(nums[j] - k) as usize] {
                    is_beautiful = false;
                    break;
                }
                found[nums[j] as usize] = true;
            }
            if is_beautiful {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2597() {
        assert_eq!(Solution::beautiful_subsets(vec![2, 4, 6], 2), 4);
        assert_eq!(Solution::beautiful_subsets(vec![1], 1), 1);
    }
}
