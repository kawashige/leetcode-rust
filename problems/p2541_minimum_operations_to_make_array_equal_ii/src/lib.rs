pub struct Solution {}

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        if k == 0 {
            if (0..nums1.len()).all(|i| nums1[i] == nums2[i]) {
                return 0;
            } else {
                return -1;
            }
        }
        let mut operations = 0;
        let mut minus = 0;
        let mut plus = 0;

        for i in 0..nums1.len() {
            if (nums1[i] - nums2[i]).abs() % k != 0 {
                return -1;
            }
            if nums1[i] < nums2[i] {
                let c = ((nums2[i] - nums1[i]) / k) as i64;
                if c <= plus {
                    plus -= c;
                } else {
                    minus += c - plus;
                    operations += c - plus;
                    plus = 0;
                }
            } else if nums1[i] > nums2[i] {
                let c = ((nums1[i] - nums2[i]) / k) as i64;
                if c <= minus {
                    minus -= c;
                } else {
                    plus += c - minus;
                    operations += c - minus;
                    minus = 0;
                }
            }
        }

        if plus == 0 && minus == 0 {
            operations
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2541() {
        assert_eq!(
            Solution::min_operations(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3),
            2
        );
        assert_eq!(
            Solution::min_operations(vec![3, 8, 5, 2], vec![2, 4, 1, 6], 1),
            -1
        );
    }
}
