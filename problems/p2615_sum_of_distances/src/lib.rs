use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut result = vec![0; nums.len()];

        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if let Some((sum, count)) = map.get(&nums[i]) {
                result[i] += count * i as i64 - sum;
            }
            let entry = map.entry(nums[i]).or_insert((0, 0));
            entry.0 += i as i64;
            entry.1 += 1;
        }
        println!("{:?}", map);

        let mut map = HashMap::new();
        for i in (0..nums.len()).rev() {
            if let Some((sum, count)) = map.get(&nums[i]) {
                result[i] += sum - count * i as i64;
            }
            let entry = map.entry(nums[i]).or_insert((0, 0));
            entry.0 += i as i64;
            entry.1 += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2615() {
        assert_eq!(Solution::distance(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
        assert_eq!(Solution::distance(vec![0, 5, 3]), vec![0, 0, 0]);
    }
}
