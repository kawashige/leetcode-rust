use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

pub struct Solution {}
impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut can_assign = BTreeSet::from_iter(0..k);
        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

        let mut assigned = vec![0; k];
        for i in 0..arrival.len() {
            while let Some((Reverse(t), j)) = heap.pop() {
                if arrival[i] < t {
                    heap.push((Reverse(t), j));
                    break;
                }
                can_assign.insert(j);
            }
            if let Some(&j) = can_assign.range(i % k..).next() {
                assigned[j] += 1;
                can_assign.remove(&j);
                heap.push((Reverse(arrival[i] + load[i]), j));
            } else if let Some(&j) = can_assign.range(..i % k).next() {
                assigned[j] += 1;
                can_assign.remove(&j);
                heap.push((Reverse(arrival[i] + load[i]), j));
            }
        }

        let max = *assigned.iter().max().unwrap();
        (0..assigned.len() as i32)
            .filter(|i| assigned[*i as usize] == max)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1606() {
        assert_eq!(
            Solution::busiest_servers(3, vec![2, 6, 7, 8], vec![1, 3, 1, 4]),
            vec![0]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
            vec![1]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
            vec![0]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
            vec![0, 1, 2]
        );
    }
}

fn main() {}
