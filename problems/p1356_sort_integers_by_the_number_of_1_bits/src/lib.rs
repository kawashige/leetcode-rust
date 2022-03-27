pub struct Solution {}

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr
            .into_iter()
            .map(|x| (x.count_ones(), x))
            .collect::<Vec<_>>();
        arr.sort_unstable();
        arr.into_iter().map(|(_, x)| x).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1356() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
