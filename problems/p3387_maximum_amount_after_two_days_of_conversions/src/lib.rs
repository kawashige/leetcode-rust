use std::collections::{HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn max_amount(
        initial_currency: String,
        pairs1: Vec<Vec<String>>,
        rates1: Vec<f64>,
        pairs2: Vec<Vec<String>>,
        rates2: Vec<f64>,
    ) -> f64 {
        let mut state = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((initial_currency.clone(), 1.0));

        while let Some((currency, amount)) = queue.pop_front() {
            if let Some(a) = state.get(&currency) {
                if &amount <= a {
                    continue;
                }
            }
            state.insert(currency.clone(), amount);

            for i in 0..pairs1.len() {
                if currency == pairs1[i][0] {
                    queue.push_back((pairs1[i][1].clone(), amount * rates1[i]));
                }
                if currency == pairs1[i][1] {
                    queue.push_back((pairs1[i][0].clone(), amount / rates1[i]));
                }
            }
        }

        let result = *state.get(&initial_currency).unwrap();
        let mut queue = VecDeque::new();
        for (currency, amount) in state {
            queue.push_back((currency, amount));
        }
        let mut state = HashMap::new();

        while let Some((currency, amount)) = queue.pop_front() {
            if let Some(a) = state.get(&currency) {
                if &amount <= a {
                    continue;
                }
            }
            state.insert(currency.clone(), amount);

            for i in 0..pairs2.len() {
                if currency == pairs2[i][0] {
                    queue.push_back((pairs2[i][1].clone(), amount * rates2[i]));
                }
                if currency == pairs2[i][1] {
                    queue.push_back((pairs2[i][0].clone(), amount / rates2[i]));
                }
            }
        }

        result.max(*state.get(&initial_currency).unwrap_or(&0.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3387() {
        assert_eq!(
            Solution::max_amount(
                "EUR".to_string(),
                vec![
                    vec!["EUR".to_string(), "USD".to_string()],
                    vec!["USD".to_string(), "JPY".to_string()]
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["JPY".to_string(), "USD".to_string()],
                    vec!["USD".to_string(), "CHF".to_string()],
                    vec!["CHF".to_string(), "EUR".to_string()]
                ],
                vec![4.0, 5.0, 6.0]
            ),
            720.00000
        );
        assert_eq!(
            Solution::max_amount(
                "NGN".to_string(),
                vec![vec!["NGN".to_string(), "EUR".to_string()]],
                vec![9.0],
                vec![vec!["NGN".to_string(), "EUR".to_string()]],
                vec![6.0]
            ),
            1.50000
        );
        assert_eq!(
            Solution::max_amount(
                "USD".to_string(),
                vec![vec!["USD".to_string(), "EUR".to_string()]],
                vec![1.0],
                vec![vec!["EUR".to_string(), "JPY".to_string()]],
                vec![10.0]
            ),
            1.00000
        );
    }
}
