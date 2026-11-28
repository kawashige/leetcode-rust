use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let mut max_factor = 0;
        let mut min_price = std::i32::MAX;
        let mut factors = HashMap::new();
        let mut items = items;
        items.sort_unstable_by_key(|items| items[1]);

        for i in 0..items.len() {
            max_factor = max_factor.max(items[i][0]);
            min_price = min_price.min(items[i][1]);

            *factors.entry(items[i][0]).or_insert(0) += 1;
        }
        let mut gains = vec![0; items.len()];
        for i in 0..items.len() {
            let mut factor = items[i][0];
            let mut gain = 0;
            if factor == 1 {
                gains[i] = items.len() as i32 - 1;
            }
            while factor <= max_factor {
                gain += factors.get(&factor).unwrap_or(&0);
                factor += items[i][0];
            }
            gains[i] = gain - 1;
        }

        let mut budget = budget;
        let mut result = 0;

        for i in 0..items.len() {
            if min_price * 2 <= items[i][1] {
                break;
            }
            if 0 < gains[i] {
                let c = (budget / items[i][1]).min(gains[i]);
                result += c * 2;
                budget -= items[i][1] * c;
            }
        }
        result + budget / min_price
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3947() {
        assert_eq!(
            Solution::maximum_sale_items(
                vec![vec![1, 45], vec![3, 9], vec![3, 43], vec![2, 30]],
                52
            ),
            6
        );
        assert_eq!(
            Solution::maximum_sale_items(
                vec![
                    vec![2, 10],
                    vec![2, 35],
                    vec![4, 19],
                    vec![4, 13],
                    vec![4, 19],
                    vec![4, 41],
                    vec![4, 10]
                ],
                71
            ),
            14
        );
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![1, 6], vec![2, 4], vec![3, 5]], 19),
            5
        );
        assert_eq!(
            Solution::maximum_sale_items(
                vec![
                    vec![2, 8],
                    vec![1, 10],
                    vec![6, 6],
                    vec![4, 12],
                    vec![5, 20],
                    vec![5, 17]
                ],
                35
            ),
            7
        );
    }
}

fn main() {}
