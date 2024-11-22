use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let mut offers = offers;
        offers.sort_unstable_by_key(|a| (a[0], a[1]));

        let mut max_gold = 0;
        let mut j = 0;
        let mut heap = BinaryHeap::new();

        for i in 0..=n {
            println!(
                "i: {}, queue: {:?}, max_gold: {}, j: {}, offres: {:?}",
                i, heap, max_gold, j, offers
            );
            while let Some((Reverse(j), g)) = heap.pop() {
                if i <= j {
                    heap.push((Reverse(j), g));
                    break;
                }
                max_gold = max_gold.max(g);
            }
            while j < offers.len() && offers[j][0] == i {
                heap.push((Reverse(offers[j][1]), offers[j][2] + max_gold));
                j += 1;
            }
        }

        max_gold
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2830() {
        assert_eq!(
            Solution::maximize_the_profit(
                15,
                vec![
                    vec![5, 5, 10],
                    vec![2, 6, 6],
                    vec![8, 11, 5],
                    vec![7, 11, 9],
                    vec![2, 4, 1],
                    vec![3, 8, 5],
                    vec![0, 6, 9],
                    vec![0, 10, 5],
                    vec![5, 10, 8],
                    vec![4, 5, 1]
                ]
            ),
            20
        );
        assert_eq!(
            Solution::maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]]),
            3
        );
        assert_eq!(
            Solution::maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 10], vec![1, 3, 2]]),
            10
        );
    }
}
