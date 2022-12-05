use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut sell = BinaryHeap::new();
        let mut buy = BinaryHeap::new();

        for order in orders {
            let mut remains = order[1];
            if order[2] == 0 {
                while let Some((Reverse(price), amount)) = sell.pop() {
                    if order[0] < price || remains == 0 {
                        sell.push((Reverse(price), amount));
                        break;
                    }
                    let executed = remains.min(amount);
                    remains -= executed;
                    if 0 < amount - executed {
                        sell.push((Reverse(price), amount - executed));
                    }
                }
                if 0 < remains {
                    buy.push((order[0], remains));
                }
            } else {
                while let Some((price, amount)) = buy.pop() {
                    if price < order[0] || remains == 0 {
                        buy.push((price, amount));
                        break;
                    }
                    let executed = remains.min(amount);
                    remains -= executed;
                    if 0 < amount - executed {
                        buy.push((price, amount - executed));
                    }
                }
                if 0 < remains {
                    sell.push((Reverse(order[0]), remains));
                }
            }
        }

        sell.into_iter()
            .map(|(_, amount)| amount)
            .chain(buy.into_iter().map(|(_, amount)| amount))
            .fold(0, |acc, amount| (acc + amount) % M)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1801() {
        assert_eq!(
            Solution::get_number_of_backlog_orders(vec![
                vec![10, 5, 0],
                vec![15, 2, 1],
                vec![25, 1, 1],
                vec![30, 4, 0]
            ]),
            6
        );
        assert_eq!(
            Solution::get_number_of_backlog_orders(vec![
                vec![7, 1000000000, 1],
                vec![15, 3, 0],
                vec![5, 999999995, 0],
                vec![5, 1, 1]
            ]),
            999999984
        );
    }
}
