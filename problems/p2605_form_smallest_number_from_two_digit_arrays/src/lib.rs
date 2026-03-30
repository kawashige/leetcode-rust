pub struct Solution {}

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if let Some(i) = (1..10).find(|i| nums1.contains(i) && nums2.contains(i)) {
            i
        } else {
            let d1 = nums1.into_iter().min().unwrap();
            let d2 = nums2.into_iter().min().unwrap();
            d1.min(d2) * 10 + d1.max(d2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2605() {
        assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7]), 15);
        assert_eq!(Solution::min_number(vec![3, 5, 2, 9], vec![3, 1, 7]), 3);
    }
}
