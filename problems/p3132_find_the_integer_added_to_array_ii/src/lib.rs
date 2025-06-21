pub struct Solution {}

impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut result = std::i32::MAX;
        for i in 0..nums1.len() {
            for j in 0..i {
                let mut k = 0;
                while k == i || k == j {
                    k += 1;
                }
                let mut d = nums2[0] - nums1[k];
                for l in 1..nums2.len() {
                    k += 1;
                    while k == i || k == j {
                        k += 1;
                    }
                    if nums2[l] - nums1[k] != d {
                        d = std::i32::MAX;
                        break;
                    }
                }
                result = result.min(d);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3132() {
        assert_eq!(
            Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]),
            -2
        );
        assert_eq!(
            Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7]),
            2
        );
    }
}
