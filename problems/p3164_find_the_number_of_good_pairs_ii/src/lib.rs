use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let count = nums2.into_iter().fold(HashMap::new(), |mut map, n| {
            *map.entry(n).or_insert(0) += 1;
            map
        });

        let mut result = 0;

        for n in nums1 {
            let sq = (n as f64).sqrt() as i32;
            for x in 1..=sq {
                if n % x != 0 {
                    continue;
                }
                if x % k == 0 {
                    result += count.get(&(x / k)).unwrap_or(&0);
                }
                if n / x != x && (n / x) % k == 0 {
                    result += count.get(&((n / x) / k)).unwrap_or(&0);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3164() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1),
            5
        );
        assert_eq!(
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3),
            2
        );
    }
}
