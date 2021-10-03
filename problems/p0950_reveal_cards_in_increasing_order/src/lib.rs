use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        let mut indices = (0..deck.len()).collect::<VecDeque<usize>>();
        let mut order = Vec::with_capacity(deck.len());
        while !indices.is_empty() {
            order.push(indices.pop_front().unwrap());
            if !indices.is_empty() {
                let tmp = indices.pop_front().unwrap();
                indices.push_back(tmp);
            }
        }

        deck.sort_unstable();

        let mut result = vec![0; deck.len()];

        for i in 0..result.len() {
            result[order[i]] = deck[i];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0950() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            vec![2, 13, 3, 11, 5, 17, 7]
        );
        assert_eq!(
            Solution::deck_revealed_increasing(vec![1, 1000]),
            vec![1, 1000]
        );
    }
}
