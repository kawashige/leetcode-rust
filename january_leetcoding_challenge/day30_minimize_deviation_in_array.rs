pub struct Solution {}

use std::collections::BTreeSet;
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut set = nums
            .into_iter()
            .map(|n| if n & 1 == 1 { n << 1 } else { n })
            .collect::<BTreeSet<i32>>();

        let mut min = *set.iter().min().unwrap();
        let mut max = *set.iter().max().unwrap();
        let mut deviation = max - min;
        while max & 1 == 0 {
            set.remove(&max);
            let mut n = max >> 1;
            while min < n >> 1 && n & 1 == 0 {
                n >>= 1;
            }
            set.insert(n);
            min = *set.iter().next().unwrap();
            max = *set.iter().next_back().unwrap();
            deviation = std::cmp::min(deviation, max - min);
        }

        deviation
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day30() {
        assert_eq!(
            Solution::minimum_deviation(vec![900, 241, 842, 374, 758, 39, 687, 242, 912]),
            609
        );
        assert_eq!(Solution::minimum_deviation(vec![4, 9, 4, 5]), 5);
        assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
        assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
        assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
    }
}
