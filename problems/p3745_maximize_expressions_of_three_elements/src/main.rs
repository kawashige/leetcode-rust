pub struct Solution {}

impl Solution {
    pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        let mut result = std::i32::MIN;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                for k in 0..nums.len() {
                    if i == k || j == k {
                        continue;
                    }
                    result = result.max(nums[i] + nums[j] - nums[k])
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3745() {
        assert_eq!(Solution::maximize_expression_of_three(vec![1, 4, 2, 5]), 8);
        assert_eq!(
            Solution::maximize_expression_of_three(vec![-2, 0, 5, -2, 4]),
            11
        );
    }
}

fn main() {}
