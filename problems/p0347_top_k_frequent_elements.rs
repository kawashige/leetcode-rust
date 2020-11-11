pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == nums.len() as i32 {
            return nums;
        }

        let mut count_map = HashMap::new();
        let mut counts = vec![0; nums.len() + 1];
        for n in nums {
            let entry = count_map.entry(n).or_insert(0);
            if entry == &0 {
                counts[1] += 1;
            } else {
                counts[*entry] -= 1;
                counts[*entry + 1] += 1;
            }
            *entry += 1;
        }

        let mut i = counts.len() - 1;
        let mut sum = 0;
        loop {
            sum += counts[i];
            if sum == k {
                break;
            }
            i -= 1;
        }

        count_map
            .into_iter()
            .filter(|(_, v)| &i <= v)
            .map(|(key, _)| key)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0347() {
        assert_eq!(
            vec![-1, 2],
            Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2)
        );
        assert_eq!(
            vec![1, 2],
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
        );
        assert_eq!(vec![1], Solution::top_k_frequent(vec![1], 1));
        assert_eq!(vec![1], Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 1));
    }
}
