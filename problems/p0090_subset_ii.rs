pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut results = HashSet::new();
        let mut bit = 0;
        while bit < (1 << nums.len()) {
            let mut v = Vec::new();
            for i in 0..nums.len() {
                if bit & (1 << i) != 0 {
                    v.push(nums[i])
                }
            }
            v.sort();
            results.insert(v);
            bit += 1;
        }
        results.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0090() {
        let mut result = Solution::subsets_with_dup(vec![1, 2, 2]);
        result.sort();
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ],
            result
        );
    }
}
