pub struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let k_count = nums.iter().filter(|x| x == &&k).count() as i32;
        let mut result = k_count;

        for x in 1..=50 {
            if x == k {
                continue;
            }
            let mut tmp_k_count = 0;
            let mut count = 0;
            let mut tmp_min = 0;

            for i in 0..nums.len() {
                if nums[i] == k {
                    tmp_k_count += 1;
                } else if nums[i] == x {
                    count += 1
                }
                result = result.max(count - tmp_min + k_count - tmp_k_count);

                if count - tmp_k_count < tmp_min {
                    tmp_min = count - tmp_k_count;
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
    fn test_3434() {
        assert_eq!(
            Solution::max_frequency(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10
                ],
                10
            ),
            100
        );
        assert_eq!(Solution::max_frequency(vec![2, 3, 7, 1, 7], 7), 3);
        assert_eq!(Solution::max_frequency(vec![2, 8], 8), 2);
        assert_eq!(Solution::max_frequency(vec![1, 2, 3, 4, 5, 6], 1), 2);
        assert_eq!(
            Solution::max_frequency(vec![10, 2, 3, 4, 5, 5, 4, 3, 2, 2], 10),
            4
        );
    }
}
