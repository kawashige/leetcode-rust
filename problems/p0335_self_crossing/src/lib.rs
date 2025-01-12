pub struct Solution {}

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        if distance.len() < 4 {
            false;
        }
        for i in 0..distance.len() {
            if (3 < i
                && (distance[i - 1] == distance[i - 3]
                    && distance[i - 2] <= distance[i - 4] + distance[i]))
                || (2 < i && (distance[i - 2] <= distance[i] && distance[i - 1] <= distance[i - 3]))
                || (4 < i
                    && (distance[i - 1] <= distance[i - 3]
                        && distance[i - 4] <= distance[i - 2]
                        && distance[i - 2] <= distance[i - 4] + distance[i]
                        && distance[i - 3] <= distance[i - 1] + distance[i - 5]))
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0335() {
        assert!(!Solution::is_self_crossing(vec![
            1, 1, 2, 2, 3, 3, 4, 4, 10, 4, 4, 3, 3, 2, 2, 1, 1
        ]));
        assert!(Solution::is_self_crossing(vec![2, 1, 4, 4, 3, 3, 2, 1, 1]));
        assert!(Solution::is_self_crossing(vec![2, 1, 1, 2]));
        assert!(!Solution::is_self_crossing(vec![1, 2, 3, 4]));
        assert!(Solution::is_self_crossing(vec![1, 1, 1, 2, 1]));
    }
}
