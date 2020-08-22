pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn generate(combination: Vec<i32>, n: i32, k: i32) -> Vec<Vec<i32>> {
            if k == 0 {
                return vec![combination];
            } else if n < k {
                return vec![];
            }
            let mut new_combination = combination.clone();
            new_combination.push(n);
            vec![
                generate(combination, n - 1, k),
                generate(new_combination, n - 1, k - 1),
            ]
            .into_iter()
            .flatten()
            .filter(|x| x.len() > 0)
            .collect::<Vec<Vec<i32>>>()
        }
        generate(Vec::new(), n, k)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0077() {
        assert_eq!(
            vec![
                vec![2, 1],
                vec![3, 1],
                vec![3, 2],
                vec![4, 1],
                vec![4, 2],
                vec![4, 3],
            ],
            Solution::combine(4, 2)
        );
        assert_eq!(vec![vec![1]], Solution::combine(1, 1));
    }
}
