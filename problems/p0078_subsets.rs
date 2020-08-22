pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut bit = 0;
        while bit < (1 << nums.len()) {
            let mut v = Vec::new();
            for i in 0..nums.len() {
                if bit & (1 << i) != 0 {
                    v.push(nums[i])
                }
            }
            results.push(v);
            bit += 1;
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0078() {
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
            Solution::subsets(vec![1, 2, 3])
        );
        assert_eq!(vec![vec![] as Vec<i32>], Solution::subsets(vec![]));
    }
}
