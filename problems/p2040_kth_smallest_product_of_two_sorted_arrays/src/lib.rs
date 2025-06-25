pub struct Solution {}

impl Solution {
    pub fn count(v: i64, x: i64, nums2: &[i64]) -> i64 {
        let n2 = nums2.len() as i64;
        let mut left = 0;
        let mut right = n2 - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let prod = x * nums2[mid as usize];
            if (x < 0 && v < prod) || (0 <= x && prod <= v) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        if 0 <= x {
            left
        } else {
            n2 - left
        }
    }

    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let nums1 = nums1.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let nums2 = nums2.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut left = -10_000_000_000;
        let mut right = 10_000_000_000;

        while left <= right {
            let mid = (left + right) / 2;
            let mut count = 0;
            for x in &nums1 {
                count += Self::count(mid, *x, &nums2);
            }
            if count < k {
                left = mid + 1;
            } else {
                right = mid - 1
            }
        }

        left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2040() {
        assert_eq!(
            Solution::kth_smallest_product(
                vec![-9, -9, -8, -6, -1, 0, 5, 8, 10, 10],
                vec![0, 4, 9],
                19
            ),
            0
        );
        assert_eq!(Solution::kth_smallest_product(vec![3], vec![-3], 1), -9);
        assert_eq!(
            Solution::kth_smallest_product(
                vec![-9, -4, 1, 6, 8, 8, 9, 10],
                vec![-10, -10, -8, -8, 1, 3, 6, 6, 8, 10],
                29
            ),
            -24
        );
        assert_eq!(
            Solution::kth_smallest_product(vec![-8, -8, 3, 7], vec![-1], 3),
            8
        );
        assert_eq!(Solution::kth_smallest_product(vec![2, 5], vec![3, 4], 2), 8);
        assert_eq!(
            Solution::kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6),
            0
        );
        assert_eq!(
            Solution::kth_smallest_product(vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3),
            -6
        );
    }
}
