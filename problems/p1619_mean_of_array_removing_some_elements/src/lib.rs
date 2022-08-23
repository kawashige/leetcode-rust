pub struct Solution {}

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();
        let l = arr.len();
        arr[l / 20..(l - l / 20)]
            .iter()
            .map(|i| *i as f64)
            .sum::<f64>()
            / (l - l / 10) as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1619() {
        assert_eq!(
            Solution::trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ]),
            2.00000
        );
        assert_eq!(
            Solution::trim_mean(vec![
                6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
            ]),
            4.00000
        );
        assert_eq!(
            Solution::trim_mean(vec![
                6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5,
                10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4
            ]),
            4.777777777777778
        );
    }
}
