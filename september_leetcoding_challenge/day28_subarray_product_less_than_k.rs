pub struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut p = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            println!("i: {}, j: {}, p: {}, nums[i]: {}", i, j, p, nums[i]);
            if p == 0 {
                if nums[i] < k {
                    p = nums[i];
                    j = i;
                    result += 1;
                } else {
                    continue;
                }
            } else {
                while k <= nums[i] * p && j < i {
                    p /= nums[j];
                    j += 1;
                }

                if nums[i] * p < k {
                    p *= nums[i];
                    result += i - j + 1;
                } else {
                    p = 0;
                    j = 0;
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day28() {
        assert_eq!(
            8,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
        );
        assert_eq!(
            0,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 2)
        );
        assert_eq!(
            4,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6, 8], 10)
        );
        assert_eq!(
            30,
            Solution::num_subarray_product_less_than_k(
                vec![6, 8, 6, 6, 10, 8, 10, 3, 7, 7, 4, 9, 3, 1],
                121
            )
        );
    }
}
