pub struct Solution {}

impl Solution {
    pub fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool {
        (start[0] + start[1]) % 2 == (target[0] + target[1]) % 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3996() {
        assert!(Solution::can_reach(vec![1, 1], vec![2, 2]));
        assert!(!Solution::can_reach(vec![4, 5], vec![6, 6]));
    }
}

fn main() {}
