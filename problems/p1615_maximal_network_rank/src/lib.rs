pub struct Solution {}

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut connected = vec![0_u128; n as usize];

        for road in roads {
            connected[road[0] as usize] |= 1 << road[1];
            connected[road[1] as usize] |= 1 << road[0];
        }

        let mut max = 0;
        for i in 0..n as usize {
            for j in 0..n as usize {
                if i != j {
                    max = max.max(
                        connected[i].count_ones() + connected[j].count_ones()
                            - if connected[i] & 1 << j != 0 { 1 } else { 0 },
                    );
                }
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1615() {
        assert_eq!(Solution::maximal_network_rank(2, vec![vec![1, 0]]), 1);
        assert_eq!(Solution::maximal_network_rank(2, vec![]), 0);
        assert_eq!(
            Solution::maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]]),
            4
        );
        assert_eq!(
            Solution::maximal_network_rank(
                5,
                vec![
                    vec![0, 1],
                    vec![0, 3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3],
                    vec![2, 4]
                ]
            ),
            5
        );
        assert_eq!(
            Solution::maximal_network_rank(
                8,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![2, 4],
                    vec![5, 6],
                    vec![5, 7]
                ]
            ),
            5
        );
    }
}
