pub struct Solution {}

impl Solution {
    pub fn score_difference(nums: Vec<i32>) -> i32 {
        let mut active = 0;
        let mut score = [0; 2];

        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                active = (active + 1) % 2;
            }
            if (i + 1) % 6 == 0 {
                active = (active + 1) % 2;
            }
            score[active] += nums[i];
        }

        score[0] - score[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3847() {
        assert_eq!(Solution::score_difference(vec![1, 2, 3]), 0);
        assert_eq!(Solution::score_difference(vec![2, 4, 2, 1, 2, 1]), 4);
        assert_eq!(Solution::score_difference(vec![1]), -1);
    }
}

fn main() {}
