pub struct Solution {}

impl Solution {
    pub fn recurse(cards: &Vec<(i32, i32)>) -> bool {
        if cards.len() == 1 {
            return cards[0].0 % cards[0].1 == 0 && cards[0].0 / cards[0].1 == 24;
        }

        for i in 0..cards.len() {
            for j in 0..cards.len() {
                if i == j {
                    continue;
                }
                let mut next_cards = Vec::new();
                for k in 0..cards.len() {
                    if k != i && k != j {
                        next_cards.push(cards[k].clone());
                    }
                }
                for op in ['+', '-', '*', '/'] {
                    let (a, b) = cards[i];
                    let (c, d) = cards[j];
                    let val = match op {
                        '+' => (a * d + b * c, b * d),
                        '-' => (a * d - b * c, b * d),
                        '*' => (a * c, b * d),
                        '/' => (a * d, b * c),
                        _ => unreachable!(),
                    };
                    if val.1 == 0 {
                        continue;
                    }
                    next_cards.push(val);
                    if Self::recurse(&next_cards) {
                        return true;
                    }
                    next_cards.pop();
                }
            }
        }
        false
    }

    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let cards = cards.into_iter().map(|c| (c, 1)).collect();
        Self::recurse(&cards)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0679() {
        assert!(Solution::judge_point24(vec![1, 3, 4, 6]));
        assert!(Solution::judge_point24(vec![4, 1, 8, 7]));
        assert!(!Solution::judge_point24(vec![1, 2, 1, 2]));
    }
}
