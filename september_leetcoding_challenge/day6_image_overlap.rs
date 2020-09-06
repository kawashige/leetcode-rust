pub struct Solution {}

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        fn indices(v: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
            let mut results = Vec::new();
            for i in 0..v.len() {
                for j in 0..v.len() {
                    if v[i][j] == 1 {
                        results.push((i as i32, j as i32));
                    }
                }
            }
            results
        }

        fn overlap(a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> i32 {
            use std::collections::HashSet;
            let mut max = 0;
            if a.len() > 0 && b.len() > 0 {
                let mut moves = HashSet::new();
                for i_a in a {
                    for i_b in b {
                        moves.insert((i_b.0 - i_a.0, i_b.1 - i_a.1));
                    }
                }
                for m in moves {
                    let mut overlap = 0;
                    for i_a in a {
                        if b.contains(&(i_a.0 + m.0, i_a.1 + m.1)) {
                            overlap += 1;
                        }
                    }
                    if max < overlap {
                        max = overlap;
                    }
                }
            }
            max
        }

        overlap(&indices(a), &indices(b))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day6() {
        assert_eq!(
            3,
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            )
        );

        assert_eq!(
            1,
            Solution::largest_overlap(vec![vec![1, 0], vec![0, 0],], vec![vec![0, 1], vec![1, 0]])
        );

        assert_eq!(
            2,
            Solution::largest_overlap(vec![vec![0, 1], vec![1, 1],], vec![vec![0, 1], vec![1, 0]])
        );

        assert_eq!(
            2,
            Solution::largest_overlap(
                vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 0, 0]],
                vec![vec![1, 0, 0], vec![1, 1, 1], vec![0, 0, 1]]
            )
        );

        assert_eq!(0, Solution::largest_overlap(vec![vec![0]], vec![vec![1]]));
        assert_eq!(0, Solution::largest_overlap(vec![vec![1]], vec![vec![0]]));

        assert_eq!(
            122,
            Solution::largest_overlap(
                vec![
                    vec![1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1],
                    vec![1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0],
                    vec![0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1],
                    vec![0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0],
                    vec![1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0],
                    vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
                    vec![1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0],
                    vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1],
                    vec![1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                    vec![1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1],
                    vec![0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
                    vec![1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1],
                    vec![0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1],
                    vec![0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1]
                ],
                vec![
                    vec![1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
                    vec![1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0],
                    vec![0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
                    vec![1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0],
                    vec![1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                    vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                    vec![1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0],
                    vec![1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]
                ]
            )
        );
    }
}
