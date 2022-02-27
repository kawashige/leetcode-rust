use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        if nums.len() % k as usize != 0 {
            return false;
        }

        let mut count = nums.iter().fold(HashMap::new(), |mut count, num| {
            *count.entry(*num).or_insert(0) += 1;
            count
        });

        nums.sort_unstable();

        for num in nums {
            if count.get(&num).unwrap_or(&0) == &0 {
                continue;
            }
            for i in 0..k {
                if let Some(c) = count.get_mut(&(num + i)) {
                    if *c == 0 {
                        return false;
                    }
                    *c -= 1;
                } else {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1296() {
        assert!(Solution::is_possible_divide(
            vec![1, 2, 3, 3, 4, 4, 5, 6],
            4
        ));
        assert!(Solution::is_possible_divide(
            vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11],
            3
        ));
        assert!(!Solution::is_possible_divide(vec![1, 2, 3, 4], 3));
        assert!(Solution::is_possible_divide(vec![3, 3, 2, 2, 1, 1], 3));
    }
}
