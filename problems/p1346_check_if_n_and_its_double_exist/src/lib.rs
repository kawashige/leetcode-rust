use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn check_if_exist(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let mut double = HashSet::new();

        for x in arr {
            if double.contains(&(if x < 0 { x * 2 } else { x })) {
                return true;
            }
            double.insert(if x < 0 { x } else { x * 2 });
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1346() {
        assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
        assert!(Solution::check_if_exist(vec![7, 1, 14, 11]));
        assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
        assert!(Solution::check_if_exist(vec![-2, -1]));
    }
}
