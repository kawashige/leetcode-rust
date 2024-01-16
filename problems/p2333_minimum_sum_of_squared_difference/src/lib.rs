pub struct Solution {}

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut diff = vec![0; 2 * 100_001];
        for i in 0..nums1.len() {
            diff[(nums1[i] - nums2[i]).abs() as usize] += 1;
        }

        let mut remains = k1 + k2;
        for i in (1..diff.len()).rev() {
            if 0 < diff[i] {
                if remains <= diff[i] {
                    diff[i - 1] += remains;
                    diff[i] -= remains;
                    break;
                } else {
                    remains -= diff[i];
                    diff[i - 1] += diff[i];
                    diff[i] = 0;
                }
            }
        }

        (0..diff.len())
            .map(|i| diff[i] as i64 * i as i64 * i as i64)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2333() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![10, 10, 10, 11, 5], vec![1, 0, 6, 6, 1], 11, 27),
            579
        );
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            579
        );
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            43
        );
    }
}
