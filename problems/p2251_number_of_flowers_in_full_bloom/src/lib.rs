use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut flowers = flowers;
        flowers.sort_unstable();

        let mut people = people.into_iter().zip(0..).collect::<Vec<_>>();
        people.sort_unstable();

        let mut result = vec![0; people.len()];
        let mut count = 0;
        let mut heap = BinaryHeap::new();
        let mut i = 0;

        for (t, p) in people {
            while i < flowers.len() && flowers[i][0] <= t {
                heap.push(Reverse(flowers[i][1]));
                count += 1;
                i += 1;
            }
            while let Some(Reverse(e)) = heap.pop() {
                if t <= e {
                    heap.push(Reverse(e));
                    break;
                }
                count -= 1;
            }
            result[p as usize] = count;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2251() {
        assert_eq!(
            Solution::full_bloom_flowers(
                vec![vec![11, 11], vec![24, 46], vec![3, 25], vec![44, 46]],
                vec![1, 8, 26, 7, 43, 26, 1]
            ),
            vec![0, 1, 1, 1, 1, 1, 0]
        );
        assert_eq!(
            Solution::full_bloom_flowers(
                vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
                vec![2, 3, 7, 11]
            ),
            vec![1, 2, 2, 2]
        );
        assert_eq!(
            Solution::full_bloom_flowers(vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2]),
            vec![2, 2, 1]
        );
    }
}
