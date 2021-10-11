pub struct Solution {}

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut nums_indices = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums_indices.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let mut ramp = 0;
        let mut min_index = nums_indices[0].0;

        for (i, _) in nums_indices {
            if i > min_index {
                ramp = ramp.max(i - min_index);
            }
            min_index = min_index.min(i);
        }

        ramp as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0962() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
    }
}
