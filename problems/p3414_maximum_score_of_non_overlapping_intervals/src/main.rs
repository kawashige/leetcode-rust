pub struct Solution {}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut arr: Vec<(i32, i32, i64, usize)> = intervals
            .iter()
            .enumerate()
            .map(|(i, interval)| (interval[1], interval[0], interval[2] as i64, i))
            .collect();
        // Sort by right endpoint.
        arr.sort_by_key(|x| x.0);

        let mut dp = vec![vec![0i64; 5]; n + 1];
        let mut indices: Vec<Vec<Vec<i32>>> = vec![vec![Vec::new(); 5]; n + 1];

        for i in 0..n {
            let (_r, l, weight, idx) = arr[i];
            // Use binary search to find intervals whose right endpoints are smaller than l.
            let k = arr[..i].partition_point(|x| x.0 < l);

            for j in 1..5 {
                let s1 = dp[i][j];
                let s2 = dp[k][j - 1] + weight;

                if s1 > s2 {
                    dp[i + 1][j] = dp[i][j];
                    indices[i + 1][j] = indices[i][j].clone();
                    continue;
                }

                let mut new_index = indices[k][j - 1].clone();
                new_index.push(idx as i32);
                new_index.sort();

                if s1 == s2 && compare_slices(&indices[i][j], &new_index) < 0 {
                    new_index = indices[i][j].clone();
                }

                dp[i + 1][j] = s2;
                indices[i + 1][j] = new_index;
            }
        }

        indices[n][4].clone()
    }
}

fn compare_slices(a: &[i32], b: &[i32]) -> i32 {
    let min_len = a.len().min(b.len());
    for i in 0..min_len {
        if a[i] != b[i] {
            return a[i] - b[i];
        }
    }
    a.len() as i32 - b.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3414() {
        assert_eq!(
            Solution::maximum_weight(vec![
                vec![1, 3, 2],
                vec![4, 5, 2],
                vec![1, 5, 5],
                vec![6, 9, 3],
                vec![6, 7, 1],
                vec![8, 9, 1]
            ]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::maximum_weight(vec![
                vec![5, 8, 1],
                vec![6, 7, 7],
                vec![4, 7, 3],
                vec![9, 10, 6],
                vec![7, 8, 2],
                vec![11, 14, 3],
                vec![3, 5, 5]
            ]),
            vec![1, 3, 5, 6]
        );
    }
}

fn main() {}
