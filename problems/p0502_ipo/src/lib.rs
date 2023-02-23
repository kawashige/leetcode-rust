use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut current_capital = w;
        let mut heap = BinaryHeap::new();
        let mut capital = capital.into_iter().zip(0..).collect::<Vec<_>>();
        capital.sort_unstable();
        let mut i = 0;

        for _ in 0..k {
            while i < capital.len() && capital[i].0 <= current_capital {
                heap.push(profits[capital[i].1]);
                i += 1;
            }

            if let Some(profit) = heap.pop() {
                current_capital += profit;
            }
        }

        current_capital
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0502() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }
}
