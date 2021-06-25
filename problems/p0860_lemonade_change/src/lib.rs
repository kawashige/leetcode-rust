pub struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        bills
            .into_iter()
            .try_fold(vec![0, 0], |mut change, pay| {
                match pay {
                    5 => change[0] += 1,
                    10 => {
                        if change[0] == 0 {
                            return None;
                        } else {
                            change[0] -= 1;
                            change[1] += 1;
                        }
                    }
                    _ => {
                        if change[0] > 0 && change[1] > 0 {
                            change[0] -= 1;
                            change[1] -= 1;
                        } else if change[0] > 2 {
                            change[0] -= 3;
                        } else {
                            return None;
                        }
                    }
                }
                Some(change)
            })
            .is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0860() {
        assert!(Solution::lemonade_change(vec![]));
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
        assert!(Solution::lemonade_change(vec![5, 5, 10]));
        assert!(!Solution::lemonade_change(vec![10, 10]));
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }
}
