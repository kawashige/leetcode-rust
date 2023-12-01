pub struct Solution {}

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut cards = cards.into_iter().zip(0..).collect::<Vec<_>>();
        cards.sort_unstable();

        let mut result = cards.len() as i32 + 1;

        for i in 1..cards.len() {
            if cards[i].0 == cards[i - 1].0 {
                result = result.min(cards[i].1 - cards[i - 1].1 + 1);
            }
        }

        if result == cards.len() as i32 + 1 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2260() {
        assert_eq!(Solution::minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
        assert_eq!(Solution::minimum_card_pickup(vec![1, 0, 5, 3]), -1);
    }
}
