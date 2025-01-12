pub struct Solution {}

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (mut sum1, mut zero1) = nums1.into_iter().fold((0, 0), |acc, num| {
            if num == 0 {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0 + num as i64, acc.1)
            }
        });
        let (mut sum2, mut zero2) = nums2.into_iter().fold((0, 0), |acc, num| {
            if num == 0 {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0 + num as i64, acc.1)
            }
        });

        if sum1 + zero1 < sum2 + zero2 {
            std::mem::swap(&mut sum1, &mut sum2);
            std::mem::swap(&mut zero1, &mut zero2);
        }

        if zero2 != 0 || sum1 + zero1 == sum2 + zero2 {
            sum1 + zero1
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2918() {
        assert_eq!(
            Solution::min_sum(
                vec![0, 16, 28, 12, 10, 15, 25, 24, 6, 0, 0],
                vec![20, 15, 19, 5, 6, 29, 25, 8, 12]
            ),
            139
        );
        assert_eq!(Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]), 12);
        assert_eq!(Solution::min_sum(vec![2, 0, 2, 0], vec![1, 4]), -1);
    }
}
