pub struct Solution {}

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut found = vec![false; 2001];
        let mut count = 0;
        for num in &nums {
            if !found[*num as usize] {
                found[*num as usize] = true;
                count += 1;
            }
        }

        let mut counts = vec![0; 2001];
        let mut tmp_count = 0;
        let mut j = 0;
        let mut result = 0;
        for i in 0..nums.len() {
            if counts[nums[i] as usize] == 0 {
                tmp_count += 1;
            }
            counts[nums[i] as usize] += 1;

            if tmp_count == count {
                while 1 < counts[nums[j] as usize] {
                    counts[nums[j] as usize] -= 1;
                    j += 1;
                }
                result += j + 1;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2799() {
        assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
        assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5, 5]), 10);
    }
}
