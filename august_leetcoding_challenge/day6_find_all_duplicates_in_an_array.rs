pub struct Solution {}

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut results = Vec::new();
        let mut count = vec![0; nums.len()];
        for v in nums.into_iter() {
            match count[v as usize - 1] {
                0 => count[v as usize - 1] += 1,
                _ => results.push(v),
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day6() {
        assert_eq!(
            vec![2, 3],
            Solution::find_duplicates(vec![4, 3, 2, 7, 2, 3, 1])
        );
    }
}
