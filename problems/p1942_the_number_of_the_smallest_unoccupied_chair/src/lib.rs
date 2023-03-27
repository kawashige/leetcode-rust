use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut times = times.into_iter().zip(0..).collect::<Vec<_>>();
        times.sort_unstable();

        let mut occupied = BinaryHeap::new();
        let mut empty = BinaryHeap::new();

        for i in 0..times.len() {
            while let Some((Reverse(leave), chair)) = occupied.pop() {
                if leave <= times[i].0[0] {
                    empty.push(Reverse(chair));
                } else {
                    occupied.push((Reverse(leave), chair));
                    break;
                }
            }
            let chair = if let Some(Reverse(empty)) = empty.pop() {
                empty
            } else {
                occupied.len() as i32
            };

            if times[i].1 == target_friend {
                return chair;
            }
            occupied.push((Reverse(times[i].0[1]), chair));
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1942() {
        assert_eq!(
            Solution::smallest_chair(
                vec![
                    vec![82057, 89365],
                    vec![32519, 49655],
                    vec![7573, 20592],
                    vec![8336, 11514],
                    vec![638, 70162],
                    vec![39511, 44262],
                    vec![70399, 79785],
                    vec![8702, 63564],
                    vec![66644, 68330],
                    vec![75156, 90448],
                    vec![11884, 87096],
                    vec![99068, 99875],
                    vec![32555, 54053],
                    vec![5910, 77572],
                    vec![87649, 94390],
                    vec![40084, 56483],
                    vec![7911, 28699],
                    vec![93308, 96154],
                    vec![25562, 39605],
                    vec![73966, 93173],
                    vec![63450, 88007],
                    vec![58811, 80045],
                    vec![56160, 71952],
                    vec![14333, 79867],
                    vec![40342, 76876],
                    vec![69943, 82175]
                ],
                5
            ),
            8
        );
        assert_eq!(
            Solution::smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1),
            1
        );
        assert_eq!(
            Solution::smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0),
            2
        );
    }
}
