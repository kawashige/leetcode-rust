use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut basket1 = basket1;
        let mut basket2 = basket2;
        basket1.sort_unstable();
        basket2.sort_unstable();

        println!("basket1: {:?}", basket1);
        println!("basket2: {:?}", basket2);

        let mut i = 0;
        let mut j = 0;
        let mut swap1 = VecDeque::new();
        let mut swap2 = VecDeque::new();

        while i < basket1.len() || j < basket2.len() {
            if ((i < basket1.len() && j < basket2.len() && basket1[i] < basket2[j])
                || (i < basket1.len() && basket2.len() <= j))
                && i + 1 < basket1.len()
                && basket1[i] == basket1[i + 1]
            {
                swap1.push_back(basket1[i] as i64);
                i += 2;
            } else if ((i < basket1.len() && j < basket2.len() && basket1[i] > basket2[j])
                || (basket1.len() <= i && j < basket2.len()))
                && j + 1 < basket2.len()
                && basket2[j] == basket2[j + 1]
            {
                swap2.push_back(basket2[j] as i64);
                j += 2;
            } else if i < basket1.len() && j < basket2.len() && basket1[i] == basket2[j] {
                i += 1;
                j += 1;
            } else {
                return -1;
            }
        }

        if swap1.len() != swap2.len() {
            return -1;
        }

        let mut cost = 0;
        let min_cost = basket1[0].min(basket2[0]) as i64 * 2;
        while !swap1.is_empty() {
            if swap1[0] < swap2[0] {
                cost += swap1[0].min(min_cost);
                swap1.pop_front();
                swap2.pop_back();
            } else {
                cost += swap2[0].min(min_cost);
                swap1.pop_back();
                swap2.pop_front();
            }
        }

        cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2561() {
        assert_eq!(
            Solution::min_cost(
                vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88],
                vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8]
            ),
            48
        );
        assert_eq!(Solution::min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
        assert_eq!(Solution::min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
    }
}
