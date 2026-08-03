pub struct Solution {}

impl Solution {
    pub fn centered_subarrays(nums: Vec<i32>) -> i32 {
        let mut acc = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            acc[i + 1] = acc[i] + nums[i];
        }

        let mut result = 0;
        for r in 0..nums.len() {
            for l in 0..=r {
                let sum = acc[r + 1] - acc[l];
                if nums[l..=r].contains(&sum) {
                    result += 1;
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
    fn test_3804() {
        assert_eq!(Solution::centered_subarrays(vec![-1, 1, 0]), 5);
        assert_eq!(Solution::centered_subarrays(vec![2, -3]), 2);
    }
}

fn main() {}
