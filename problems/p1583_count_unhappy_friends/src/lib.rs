pub struct Solution {}

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let mut rank = vec![vec![n as usize + 1; n as usize]; n as usize];
        for i in 0..n as usize {
            for j in 0..(n - 1) as usize {
                rank[i][preferences[i][j] as usize] = j;
            }
        }

        let mut pair = vec![0; n as usize];
        for p in pairs {
            pair[p[0] as usize] = p[1] as usize;
            pair[p[1] as usize] = p[0] as usize;
        }

        let mut count = 0;

        for i in 0..n as usize {
            for p in &preferences[i] {
                if p == &(pair[i] as i32) {
                    break;
                }

                if rank[*p as usize][i] < rank[*p as usize][pair[*p as usize]] {
                    count += 1;
                    break;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1583() {
        assert_eq!(
            Solution::unhappy_friends(
                4,
                vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]],
                vec![vec![0, 1], vec![2, 3]]
            ),
            2
        );
        assert_eq!(
            Solution::unhappy_friends(2, vec![vec![1], vec![0]], vec![vec![1, 0]]),
            0
        );
        assert_eq!(
            Solution::unhappy_friends(
                4,
                vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]],
                vec![vec![1, 3], vec![0, 2]]
            ),
            4
        );
    }
}
