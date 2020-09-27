pub struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        fn recurse(
            start: usize,
            end: usize,
            current: f64,
            graph: &Vec<Vec<f64>>,
            visited: Vec<usize>,
        ) -> f64 {
            if graph[start][end] != -1.0 {
                return current * graph[start][end];
            }

            for (i, v) in graph[start].iter().enumerate().filter(|(_, v)| **v != -1.0) {
                if visited.contains(&i) {
                    continue;
                }
                let mut new_visited = visited.clone();
                new_visited.push(i);
                let result = recurse(i, end, current * v, graph, new_visited);
                if result != -1.0 {
                    return result;
                }
            }

            -1.0
        }

        let mut results = Vec::new();

        let mut nodes = Vec::new();
        for e in &equations {
            for elm in e {
                if !nodes.contains(elm) {
                    nodes.push(elm.clone());
                }
            }
        }

        let l = nodes.len();
        let mut graph = vec![vec![-1.0; l]; l];
        for i in 0..l {
            graph[i][i] = 1.0;
        }
        for i in 0..equations.len() {
            let i_1 = nodes.iter().position(|n| n == &equations[i][0]).unwrap();
            let i_2 = nodes.iter().position(|n| n == &equations[i][1]).unwrap();
            graph[i_1][i_2] = values[i];
            graph[i_2][i_1] = 1.0 / values[i];
        }

        // println!("{:?}", graph);

        for q in queries {
            let i_1 = nodes.iter().position(|n| n == &q[0]);
            let i_2 = nodes.iter().position(|n| n == &q[1]);
            results.push(if i_1.is_none() || i_2.is_none() {
                -1.0
            } else {
                recurse(i_1.unwrap(), i_2.unwrap(), 1.0, &graph, vec![i_1.unwrap()])
            });
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day27() {
        assert_eq!(
            vec![3.75000, 0.40000, 5.00000, 0.20000],
            Solution::calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()],
                    vec!["bc".to_string(), "cd".to_string()]
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["c".to_string(), "b".to_string()],
                    vec!["bc".to_string(), "cd".to_string()],
                    vec!["cd".to_string(), "bc".to_string()]
                ]
            )
        );

        assert_eq!(
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            Solution::calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()]
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "e".to_string()],
                    vec!["a".to_string(), "a".to_string()],
                    vec!["x".to_string(), "x".to_string()]
                ]
            )
        );

        assert_eq!(
            vec![0.50000, 2.00000, -1.00000, -1.00000],
            Solution::calc_equation(
                vec![vec!["a".to_string(), "b".to_string()],],
                vec![0.5],
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "c".to_string()],
                    vec!["x".to_string(), "y".to_string()]
                ]
            )
        );
    }
}
