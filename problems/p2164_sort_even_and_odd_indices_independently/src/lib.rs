pub struct Solution {}

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut odd = vec![0; 101];
        let mut even = vec![0; 101];

        for i in 0..nums.len() {
            if i % 2 == 0 {
                odd[nums[i] as usize] += 1;
            } else {
                even[nums[i] as usize] += 1;
            }
        }

        let mut j = 0;
        for i in (0..nums.len()).step_by(2) {
            while odd[j] == 0 {
                j += 1;
            }
            odd[j] -= 1;
            nums[i] = j as i32;
        }
        let mut j = 100;
        for i in (1..nums.len()).step_by(2) {
            while even[j] == 0 {
                j -= 1;
            }
            even[j] -= 1;
            nums[i] = j as i32;
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2164() {
        assert_eq!(Solution::sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
        assert_eq!(Solution::sort_even_odd(vec![2, 1]), vec![2, 1]);
    }
}
