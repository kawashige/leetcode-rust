pub struct Solution {}

impl Solution {
    pub fn count(nums: &[i32], value: i32) -> usize {
        if nums.is_empty() || value < nums[0] {
            return 0;
        }
        let mut ok = 0;
        let mut ng = nums.len();

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if nums[mid] <= value {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok + 1
    }

    pub fn find2(nums1: &[i32], nums2: &[i32], count: usize) -> Option<i32> {
        if nums1.is_empty() {
            return None;
        }
        if count <= Self::count(nums2, nums1[0]) {
            return Some(nums1[0]);
        }

        let mut ng = 0;
        let mut ok = nums1.len();

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if count <= mid + Self::count(nums2, nums1[mid]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        if ok == nums1.len() {
            None
        } else {
            Some(nums1[ok])
        }
    }

    pub fn find(nums1: &[i32], nums2: &[i32], count: usize) -> i32 {
        match (
            Self::find2(&nums1, &nums2, count),
            Self::find2(&nums2, &nums1, count),
        ) {
            (Some(n1), Some(n2)) => {
                if n1 <= n2 {
                    n1
                } else {
                    n2
                }
            }
            (Some(n1), None) => n1,
            (None, Some(n2)) => n2,
            _ => unreachable!(),
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if (nums1.len() + nums2.len()) % 2 == 1 {
            Self::find(&nums1, &nums2, (nums1.len() + nums2.len()) / 2) as f64
        } else {
            (Self::find(&nums1, &nums2, (nums1.len() + nums2.len()) / 2) as f64
                + Self::find(&nums1, &nums2, (nums1.len() + nums2.len()) / 2 - 1) as f64)
                / 2.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0004() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]),
            0.00000
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1]),
            1.00000
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.00000
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.50000
        );
    }
}
