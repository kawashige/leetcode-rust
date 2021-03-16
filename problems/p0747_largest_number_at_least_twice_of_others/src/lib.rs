pub struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut v = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
        v.sort_unstable_by_key(|(_, x)| -x);
        if v.len() == 1 || v[0].1 >= v[1].1 * 2 {
            v[0].0 as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0747() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }
}
