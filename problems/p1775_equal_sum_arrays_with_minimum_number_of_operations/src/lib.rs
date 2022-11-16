pub struct Solution {}

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sum1: i32 = nums1.iter().sum();
        let sum2: i32 = nums2.iter().sum();

        if sum1 == sum2 {
            return 0;
        }

        let (mut large, mut sum_large, mut small, mut sum_small) = if sum1 < sum2 {
            (nums2, sum2, nums1, sum1)
        } else {
            (nums1, sum1, nums2, sum2)
        };

        large.sort_unstable_by(|a, b| b.cmp(&a));
        small.sort_unstable();

        let mut i_large = 0;
        let mut i_small = 0;
        let mut count = 0;

        while sum_small != sum_large {
            let d_large = if large.len() <= i_large {
                0
            } else {
                large[i_large] - 1
            };
            let d_small = if small.len() <= i_small {
                0
            } else {
                6 - small[i_small]
            };

            if d_large == 0 && d_small == 0 {
                return -1;
            }

            if d_large < d_small {
                i_small += 1;
                sum_small += d_small.min(sum_large - sum_small);
            } else {
                i_large += 1;
                sum_large -= d_large.min(sum_large - sum_small);
            }
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1775() {
        assert_eq!(
            Solution::min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2]),
            3
        );
        assert_eq!(
            Solution::min_operations(vec![1, 1, 1, 1, 1, 1, 1], vec![6]),
            -1
        );
        assert_eq!(Solution::min_operations(vec![6, 6], vec![1]), 3);
    }
}
