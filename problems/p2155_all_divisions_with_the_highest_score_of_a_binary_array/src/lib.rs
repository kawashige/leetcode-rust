pub struct Solution {}

impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let all_ones = nums.iter().filter(|num| num == &&1).count();
        let mut highest = all_ones;
        let mut result = vec![0];
        let mut zeros = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                zeros += 1;
            }

            let score = zeros + all_ones - (i + 1 - zeros);
            if highest == score {
                result.push(i as i32 + 1);
            } else if highest < score {
                highest = score;
                result = vec![i as i32 + 1];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2155() {
        assert_eq!(Solution::max_score_indices(vec![0, 0, 1, 0]), vec![2, 4]);
        assert_eq!(Solution::max_score_indices(vec![0, 0, 0]), vec![3]);
        assert_eq!(Solution::max_score_indices(vec![1, 1]), vec![0]);
    }
}
