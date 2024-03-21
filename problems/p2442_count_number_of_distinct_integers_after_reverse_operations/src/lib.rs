use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn reverse(num: i32) -> i32 {
        let mut num = num;
        let mut reversed = 0;
        while 0 < num {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        reversed
    }

    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for num in nums {
            set.insert(Self::reverse(num));
            set.insert(num);
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2442() {
        assert_eq!(
            Solution::count_distinct_integers(vec![1, 13, 10, 12, 31]),
            6
        );
        assert_eq!(Solution::count_distinct_integers(vec![2, 2, 2]), 1);
    }
}
