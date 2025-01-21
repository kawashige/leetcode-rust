pub struct Solution {}

impl Solution {
    pub fn recurce(cur: usize, prev: usize, list: &Vec<Vec<usize>>, values: &[i32]) -> (i64, i64) {
        let mut dp = 0;
        let mut sum = 0;
        for n in &list[cur] {
            if n == &prev {
                continue;
            }
            let result = Self::recurce(*n, cur, list, values);
            dp += result.0;
            sum += result.1;
        }

        if sum == 0 {
            (0, values[cur] as i64)
        } else {
            ((values[cur] as i64 + dp).max(sum), sum + values[cur] as i64)
        }
    }
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let mut list = vec![vec![]; values.len()];
        for e in edges {
            list[e[0] as usize].push(e[1] as usize);
            list[e[1] as usize].push(e[0] as usize);
        }

        Self::recurce(0, std::usize::MAX, &list, &values).0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2925() {
        assert_eq!(
            Solution::maximum_score_after_operations(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![4, 5]],
                vec![5, 2, 5, 2, 1, 1]
            ),
            11
        );
        assert_eq!(
            Solution::maximum_score_after_operations(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![20, 10, 9, 7, 4, 3, 5]
            ),
            40
        );
    }
}
