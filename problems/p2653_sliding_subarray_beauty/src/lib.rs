pub struct Solution {}

impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut count = vec![0; 101];
        let mut result = Vec::with_capacity(nums.len() - k as usize + 1);

        for i in 0..nums.len() {
            count[(nums[i] + 50) as usize] += 1;
            if k <= i as i32 {
                count[(nums[i - k as usize] + 50) as usize] -= 1;
            }
            if i < k as usize - 1 {
                continue;
            }
            let mut remains = x;
            for j in 0..count.len() {
                if 0 < count[j] {
                    remains -= remains.min(count[j]);
                    if remains == 0 {
                        result.push((j as i32 - 50).min(0));
                        break;
                    }
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
    fn test_2653() {
        assert_eq!(
            Solution::get_subarray_beauty(vec![1, -1, -3, -2, 3], 3, 2),
            vec![-1, -2, -2]
        );
        assert_eq!(
            Solution::get_subarray_beauty(vec![-1, -2, -3, -4, -5], 2, 2),
            vec![-1, -2, -3, -4]
        );
        assert_eq!(
            Solution::get_subarray_beauty(vec![-3, 1, 2, -3, 0, -3], 2, 1),
            vec![-3, 0, -3, -3, -3]
        );
    }
}
