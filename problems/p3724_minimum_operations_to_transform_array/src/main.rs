pub struct Solution {}

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let append_num = nums2[nums2.len() - 1];
        let mut append_min_dist = std::i32::MAX;
        let mut result = 1;

        for i in 0..nums1.len() {
            result += (nums1[i] - nums2[i]).abs() as i64;
            if (nums1[i].min(nums2[i])..=nums1[i].max(nums2[i])).contains(&append_num) {
                append_min_dist = 0;
            } else {
                append_min_dist = append_min_dist
                    .min((append_num - nums1[i]).abs())
                    .min((append_num - nums2[i]).abs());
            }
        }

        result + append_min_dist as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3724() {
        assert_eq!(Solution::min_operations(vec![2, 8], vec![1, 7, 3]), 4);
        assert_eq!(Solution::min_operations(vec![1, 3, 6], vec![2, 4, 5, 3]), 4);
        assert_eq!(Solution::min_operations(vec![2], vec![3, 4]), 3);
    }
}

fn main() {}
