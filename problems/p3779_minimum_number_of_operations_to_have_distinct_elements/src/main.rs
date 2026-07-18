pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count = [0; 100_001];
        let mut duplicate_count = 0;
        for num in &nums {
            if count[*num as usize] == 1 {
                duplicate_count += 1;
            }
            count[*num as usize] += 1;
        }

        let mut i = 0;
        let mut result = 0;
        while i < nums.len() && 0 < duplicate_count {
            result += 1;
            for j in 0..3 {
                if nums.len() <= i + j {
                    break;
                }
                count[nums[i + j] as usize] -= 1;
                if count[nums[i + j] as usize] == 1 {
                    duplicate_count -= 1;
                }
            }
            i += 3;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3779() {
        assert_eq!(Solution::min_operations(vec![3, 8, 3, 6, 5, 8]), 1);
        assert_eq!(Solution::min_operations(vec![2, 2]), 1);
        assert_eq!(Solution::min_operations(vec![4, 3, 5, 1, 2]), 0);
    }
}

fn main() {}
