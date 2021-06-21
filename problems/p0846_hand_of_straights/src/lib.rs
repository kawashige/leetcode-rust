use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        if group_size == 1 {
            return true;
        }

        hand.sort_unstable();

        let mut group = VecDeque::new();

        for h in hand {
            if group.is_empty() {
                group.push_back((h, 1));
            } else {
                if group[0].0 + 1 == h {
                    if group[0].1 == group_size - 1 {
                        group.pop_front();
                    } else {
                        group[0] = (h, group[0].1 + 1);
                    }
                } else if group[0].0 == h {
                    if let Some(j) = (0..group.len()).skip_while(|j| group[*j].0 == h).next() {
                        if group[j].0 == h - 1 {
                            group[j] = (h, group[j].1 + 1);
                        } else {
                            return false;
                        }
                    } else {
                        group.push_back((h, 1));
                    }
                } else {
                    return false;
                }
            }
        }

        group.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0846() {
        assert!(!Solution::is_n_straight_hand(vec![1, 1, 1, 1, 1, 1], 2));
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }
}
