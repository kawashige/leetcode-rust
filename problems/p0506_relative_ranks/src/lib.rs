pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut sorted = nums.clone();
        sorted.sort_by_key(|n| -n);
        let map = sorted.into_iter().zip(1..).collect::<HashMap<i32, usize>>();
        nums.iter()
            .map(|n| {
                let rank = map.get(&n).unwrap();
                match rank {
                    1 => "Gold Medal".to_string(),
                    2 => "Silver Medal".to_string(),
                    3 => "Bronze Medal".to_string(),
                    _ => rank.to_string(),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0506() {
        assert_eq!(
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ],
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            vec![
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "Gold Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ],
            Solution::find_relative_ranks(vec![4, 3, 5, 2, 1])
        );
        assert_eq!(vec![] as Vec<String>, Solution::find_relative_ranks(vec![]));
    }
}
