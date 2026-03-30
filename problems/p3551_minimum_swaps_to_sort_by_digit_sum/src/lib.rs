use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let mut remains = *n;
                let mut x = 0;
                while 0 < remains {
                    x += remains % 10;
                    remains /= 10;
                }
                (x, *n, i)
            })
            .collect::<Vec<_>>();
        sorted_nums.sort_unstable();

        let mut map = HashMap::new();
        let mut nums = nums;
        for i in 0..nums.len() {
            map.insert(nums[i], i);
        }

        let mut count = 0;
        for i in 0..sorted_nums.len() {
            if nums[i] == sorted_nums[i].1 {
                continue;
            }
            let j = *map.get(&sorted_nums[i].1).unwrap();
            count += 1;
            nums[j] = nums[i];
            map.insert(nums[i], j);
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3551() {
        assert_eq!(Solution::min_swaps(vec![37, 100]), 1);
        assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0);
        assert_eq!(Solution::min_swaps(vec![18, 43, 34, 16]), 2);
    }
}
