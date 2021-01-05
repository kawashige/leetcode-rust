pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut results = vec![vec![vec![nums[0]]]];
        let mut exists = HashSet::new();
        for i in 1..nums.len() {
            let s = if nums[i - 1] == nums[i] { i - 1 } else { 0 };
            let mut tmp = vec![vec![nums[i]]];
            for j in (s..i).filter(|k| nums[*k] <= nums[i]) {
                for v in &results[j] {
                    let mut new = v.clone();
                    new.push(nums[i]);
                    let key = new
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    if !exists.contains(&key) {
                        tmp.push(new);
                        exists.insert(key);
                    }
                }
            }
            results.push(tmp);
        }

        results
            .into_iter()
            .flatten()
            .filter(|v| 1 < v.len())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0491() {
        assert_eq!(
            vec![
                vec![4, 6],
                vec![4, 7],
                vec![6, 7],
                vec![4, 6, 7],
                vec![7, 7],
                vec![4, 7, 7],
                vec![6, 7, 7],
                vec![4, 6, 7, 7],
            ],
            Solution::find_subsequences(vec![4, 6, 7, 7])
        );
        assert_eq!(
            vec![] as Vec<Vec<i32>>,
            Solution::find_subsequences(vec![4, 3, 2, 1])
        );
        assert_eq!(
            vec![
                vec![1, 1],
                vec![1, 1, 1],
                vec![0, 1],
                vec![1, 1, 1, 1],
                vec![0, 1, 1]
            ],
            Solution::find_subsequences(vec![1, 1, 0, 1, 1])
        );
    }
}
