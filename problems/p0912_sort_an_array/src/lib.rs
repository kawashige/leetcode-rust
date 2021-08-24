pub struct Solution {}

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        const BASE: usize = 50_000;
        let mut result = Vec::with_capacity(nums.len());
        let mut count = vec![0; BASE * 2 + 1];
        for x in nums {
            count[(x + BASE as i32) as usize] += 1
        }
        for i in 0..count.len() {
            for _ in 0..count[i] {
                result.push(i as i32 - BASE as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0912() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }
}
