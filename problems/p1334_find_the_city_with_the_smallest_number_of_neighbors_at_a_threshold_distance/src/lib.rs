pub struct Solution {}

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut dist = vec![vec![distance_threshold + 1; n as usize]; n as usize];

        for i in 0..n as usize {
            dist[i][i] = 0;
        }

        for edge in edges {
            dist[edge[0] as usize][edge[1] as usize] = edge[2];
            dist[edge[1] as usize][edge[0] as usize] = edge[2];
        }

        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        (0..n)
            .map(|i| {
                (
                    dist[i as usize]
                        .iter()
                        .filter(|x| x <= &&distance_threshold)
                        .count(),
                    -i,
                )
            })
            .min()
            .unwrap()
            .1
            * -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1334() {
        assert_eq!(
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        );
        assert_eq!(
            Solution::find_the_city(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 4, 8],
                    vec![1, 2, 3],
                    vec![1, 4, 2],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                2
            ),
            0
        );
    }
}
