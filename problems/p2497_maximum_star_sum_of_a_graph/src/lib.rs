pub struct Solution {}

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut neighbours_vals = vals.clone();
        let mut vals = vals.into_iter().zip(0..).collect::<Vec<_>>();
        vals.sort_unstable_by(|a, b| b.cmp(&a));
        let mut edge_count = vec![0; vals.len()];

        let mut list = vec![vec![]; vals.len()];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        for i in 0..vals.len() {
            if vals[i].0 <= 0 {
                break;
            }
            for node in &list[vals[i].1] {
                if edge_count[*node] < k {
                    neighbours_vals[*node] += vals[i].0;
                    edge_count[*node] += 1;
                }
            }
        }

        neighbours_vals.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2497() {
        assert_eq!(
            Solution::max_star_sum(
                vec![0, -36, 4, 35, 27, -13],
                vec![vec![5, 3], vec![4, 3], vec![0, 4], vec![2, 4], vec![0, 2]],
                1
            ),
            62
        );
        assert_eq!(
            Solution::max_star_sum(
                vec![1, 2, 3, 4, 10, -10, -20],
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![1, 3],
                    vec![3, 4],
                    vec![3, 5],
                    vec![3, 6]
                ],
                2
            ),
            16
        );
        assert_eq!(Solution::max_star_sum(vec![-5], vec![], 0), -5);
    }
}
