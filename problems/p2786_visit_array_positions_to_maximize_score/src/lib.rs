pub struct Solution {}

impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let mut max = vec![std::i64::MIN; 2];
        max[nums[0] as usize % 2] = nums[0] as i64;

        for i in 1..nums.len() {
            if nums[i] % 2 == 0 {
                if max[0] != std::i64::MIN {
                    max[0] += nums[i] as i64;
                }
                if max[1] != std::i64::MIN {
                    max[0] = max[0].max(max[1] + (nums[i] - x) as i64)
                }
            } else {
                if max[1] != std::i64::MIN {
                    max[1] += nums[i] as i64;
                }
                if max[0] != std::i64::MIN {
                    max[1] = max[1].max(max[0] + (nums[i] - x) as i64)
                }
            }
        }

        max[0].max(max[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2786() {
        assert_eq!(
            Solution::max_score(
                vec![
                    9, 58, 17, 54, 91, 90, 32, 6, 13, 67, 24, 80, 8, 56, 29, 66, 85, 38, 45, 13,
                    20, 73, 16, 98, 28, 56, 23, 2, 47, 85, 11, 97, 72, 2, 28, 52, 33
                ],
                90
            ),
            886
        );
        assert_eq!(Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
        assert_eq!(Solution::max_score(vec![2, 4, 6, 8], 3), 20);
    }
}
