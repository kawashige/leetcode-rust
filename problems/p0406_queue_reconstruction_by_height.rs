pub struct Solution {}

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted = people;
        sorted.sort_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));

        let mut results = Vec::new();
        for v in sorted {
            results.insert(v[1] as usize, v);
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0406() {
        assert_eq!(
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ],
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ])
        );
        assert_eq!(
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0]
            ],
            Solution::reconstruct_queue(vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4]
            ])
        );
    }
}
