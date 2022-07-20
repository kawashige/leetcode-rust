pub struct Solution {}

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut in_count = vec![0; n as usize];
        for edge in edges {
            in_count[edge[1] as usize] += 1;
        }
        (0..n).filter(|i| in_count[*i as usize] == 0).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1557() {
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]
            ),
            vec![0, 3]
        );
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                5,
                vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]]
            ),
            vec![0, 2, 3]
        );
    }
}
