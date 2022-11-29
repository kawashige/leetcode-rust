use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Debug)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Solution {}

impl Solution {
    pub fn change(pass: i32, total: i32) -> f64 {
        if pass == total {
            return 0.0;
        }
        let pass = pass as f64;
        let total = total as f64;
        (total - pass) / (total * (total + 1.0))
    }

    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();
        for class in classes {
            heap.push((
                MinNonNan(Self::change(class[0], class[1])),
                class[0],
                class[1],
            ));
        }

        for _ in 0..extra_students {
            if let Some((_, pass, total)) = heap.pop() {
                heap.push((
                    MinNonNan(Self::change(pass + 1, total + 1)),
                    pass + 1,
                    total + 1,
                ));
            }
        }

        let count = heap.len();
        heap.into_iter()
            .map(|(_, pass, total)| pass as f64 / total as f64)
            .sum::<f64>()
            / count as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1792() {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.7833333333333333
        );
        assert_eq!(
            Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
            0.5348484848484848
        );
    }
}
