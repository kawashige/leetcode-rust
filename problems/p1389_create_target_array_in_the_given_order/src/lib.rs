pub struct Solution {}

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        index
            .into_iter()
            .enumerate()
            .fold(Vec::new(), |mut v, (i, j)| {
                v.insert(j as usize, nums[i]);
                v
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1389() {
        assert_eq!(
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
    }
}
