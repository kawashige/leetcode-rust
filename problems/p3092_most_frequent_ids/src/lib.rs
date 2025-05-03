use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
        let mut count = vec![0; 100_001];
        let mut map = BTreeMap::new();
        let mut result = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            if count[nums[i] as usize] != 0 {
                *map.get_mut(&count[nums[i] as usize]).unwrap() -= 1;
                if map[&count[nums[i] as usize]] == 0 {
                    map.remove(&count[nums[i] as usize]);
                }
            }
            count[nums[i] as usize] += freq[i] as i64;
            *map.entry(count[nums[i] as usize]).or_insert(0) += 1;
            let (key, _) = map.iter().next_back().unwrap_or((&0, &0));
            result.push(*key);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3092() {
        assert_eq!(
            Solution::most_frequent_i_ds(vec![2, 3, 2, 1], vec![3, 2, -3, 1]),
            vec![3, 3, 2, 2]
        );
        assert_eq!(
            Solution::most_frequent_i_ds(vec![5, 5, 3], vec![2, -2, 1]),
            vec![2, 0, 1]
        );
    }
}
