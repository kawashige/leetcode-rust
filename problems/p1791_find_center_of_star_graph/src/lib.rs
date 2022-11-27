pub struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; edges.len() + 2];
        for edge in &edges {
            count[edge[0] as usize] += 1;
            count[edge[1] as usize] += 1;
        }

        (1..count.len()).find(|i| count[*i] == edges.len()).unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1791() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
