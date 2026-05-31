use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        for i in 1.. {
            if !nums.contains(&(i * k)) {
                return i * k;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3718() {
        assert_eq!(Solution::missing_multiple(vec![8, 2, 3, 4, 6], 2), 10);
        assert_eq!(Solution::missing_multiple(vec![1, 4, 7, 10, 15], 5), 5);
    }
}

fn main() {}
