use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let (odd, even) = (0..nums.len()).fold(
            (HashMap::new(), HashMap::new()),
            |(mut odd, mut even), i| {
                if i % 2 == 0 {
                    *odd.entry(nums[i]).or_insert(0) += 1;
                } else {
                    *even.entry(nums[i]).or_insert(0) += 1;
                }
                (odd, even)
            },
        );

        let mut odd = odd.into_iter().collect::<Vec<_>>();
        let mut even = even.into_iter().collect::<Vec<_>>();
        odd.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        even.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        (if even[0].0 != odd[0].0 {
            nums.len() - even[0].1 - odd[0].1
        } else {
            nums.len()
                - (odd[0].1 + if even.len() == 1 { 0 } else { even[1].1 })
                    .max(if odd.len() == 1 { 0 } else { odd[1].1 } + even[0].1)
        }) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2170() {
        assert_eq!(Solution::minimum_operations(vec![3, 1, 3, 2, 4, 3]), 3);
        assert_eq!(Solution::minimum_operations(vec![1, 2, 2, 2, 2]), 2);
    }
}
