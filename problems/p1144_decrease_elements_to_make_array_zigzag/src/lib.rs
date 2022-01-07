pub struct Solution {}

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        [0, 1]
            .iter()
            .map(|j| {
                (*j..nums.len())
                    .step_by(2)
                    .map(|i| {
                        (nums[i]
                            - (if i == 0 { nums[i] } else { nums[i - 1] - 1 }).min(
                                if i == nums.len() - 1 {
                                    nums[i]
                                } else {
                                    nums[i + 1] - 1
                                },
                            ))
                        .max(0)
                    })
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1144() {
        assert_eq!(Solution::moves_to_make_zigzag(vec![3]), 0);
        assert_eq!(Solution::moves_to_make_zigzag(vec![1, 2, 3]), 2);
        assert_eq!(Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
    }
}
