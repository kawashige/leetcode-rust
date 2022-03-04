pub struct Solution {}

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xor = vec![0; arr.len() + 1];
        for i in 0..arr.len() {
            xor[i + 1] = xor[i] ^ arr[i];
        }

        queries
            .into_iter()
            .map(|query| xor[query[1] as usize + 1] ^ xor[query[0] as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1310() {
        assert_eq!(
            Solution::xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
            ),
            vec![2, 7, 14, 8]
        );
        assert_eq!(
            Solution::xor_queries(
                vec![4, 8, 2, 10],
                vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]
            ),
            vec![8, 0, 4, 4]
        );
    }
}
