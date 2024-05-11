use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers = quality
            .into_iter()
            .zip(wage.into_iter())
            .collect::<Vec<_>>();
        workers.sort_unstable_by(|a, b| (a.1 * b.0).cmp(&(b.1 * a.0)));

        let mut heap = BinaryHeap::new();
        let mut total_quality = 0;
        let mut min_money = std::f64::MAX;

        for i in 0..workers.len() {
            if k as usize - 1 <= heap.len() {
                min_money = min_money.min(
                    (total_quality + workers[i].0) as f64
                        * (workers[i].1 as f64 / workers[i].0 as f64),
                );
            }
            heap.push(workers[i].0);
            total_quality += workers[i].0;
            if heap.len() == k as usize {
                total_quality -= heap.pop().unwrap();
            }
        }

        min_money
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0857() {
        assert_eq!(
            Solution::mincost_to_hire_workers(vec![2, 1, 5], vec![17, 6, 4], 2),
            25.50000
        );
        assert_eq!(
            Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2),
            105.00000
        );
        assert_eq!(
            Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3),
            30.666666666666664
        );
    }
}
