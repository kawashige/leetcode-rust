pub struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        arr.into_iter()
            .enumerate()
            .fold((0, 0), |(max, chunks), (i, n)| {
                let max = std::cmp::max(n, max);
                (max, chunks + if i == max as usize { 1 } else { 0 })
            })
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0769() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
