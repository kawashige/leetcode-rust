pub struct Solution {}

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut marked = vec![false; nums.len()];
        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut score = 0;

        for i in 0..nums.len() {
            if marked[nums[i].1] {
                continue;
            }
            score += nums[i].0 as i64;
            marked[nums[i].1] = true;
            if 0 < nums[i].1 {
                marked[nums[i].1 - 1] = true;
            }
            if nums[i].1 < nums.len() - 1 {
                marked[nums[i].1 + 1] = true;
            }
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2593() {
        assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
        assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);
    }
}
