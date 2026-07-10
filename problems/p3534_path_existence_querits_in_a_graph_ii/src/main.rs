use std::mem::swap;

pub struct Solution {}

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut indices = vec![0; n as usize];
        for i in 0..n {
            indices[nums[i].1] = i;
        }

        let mut st = vec![vec![0; 18]; n];
        let mut r = 0;
        for i in 0..n {
            if r < i {
                r = i;
            }
            while r + 1 < n
                && nums[r + 1].0 - nums[r].0 <= max_diff
                && nums[r + 1].0 - nums[i].0 <= max_diff
            {
                r += 1;
            }
            st[i][0] = r;
        }

        for j in 1..18 {
            for i in 0..n {
                st[i][j] = st[st[i][j - 1]][j - 1];
            }
        }

        queries
            .into_iter()
            .map(|q| {
                let mut a = indices[q[0] as usize];
                let mut b = indices[q[1] as usize];
                if b < a {
                    swap(&mut a, &mut b);
                }
                if a == b {
                    return 0;
                }
                let mut cur = a;
                let mut steps = 0;
                for j in (0..18).rev() {
                    if st[cur][j] < b {
                        cur = st[cur][j];
                        steps += 1 << j;
                    }
                }
                if b <= st[cur][0] { steps + 1 } else { -1 }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3543() {
        assert_eq!(
            Solution::path_existence_queries(
                5,
                vec![1, 8, 3, 4, 2],
                3,
                vec![vec![0, 3], vec![2, 4]]
            ),
            vec![1, 1]
        );
        assert_eq!(
            Solution::path_existence_queries(
                5,
                vec![5, 3, 1, 9, 10],
                2,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]]
            ),
            vec![1, 2, -1, 1]
        );
        assert_eq!(
            Solution::path_existence_queries(
                3,
                vec![3, 6, 1],
                1,
                vec![vec![0, 0], vec![0, 1], vec![1, 2]]
            ),
            vec![0, -1, -1]
        );
    }
}

fn main() {}
