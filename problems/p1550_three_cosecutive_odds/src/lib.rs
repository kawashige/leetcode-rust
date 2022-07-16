pub struct Solution {}

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut odds = 0;
        for x in arr {
            if x % 2 == 1 {
                odds += 1;
                if odds == 3 {
                    return true;
                }
            } else {
                odds = 0;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1550() {
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
    }
}
