pub struct Solution {}

impl Solution {
    pub fn smallest_range_ii(a: Vec<i32>, k: i32) -> i32 {
        let mut a = a;
        a.sort();
        let mut result = a[a.len() - 1] - a[0];
        for i in 0..(a.len() - 1) {
            let max = std::cmp::max(a[a.len() - 1] - k, a[i] + k);
            let min = std::cmp::min(a[0] + k, a[i + 1] - k);
            result = std::cmp::min(result, max - min);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day21() {
        assert_eq!(0, Solution::smallest_range_ii(vec![1], 0));
        assert_eq!(6, Solution::smallest_range_ii(vec![0, 10], 2));
        assert_eq!(3, Solution::smallest_range_ii(vec![1, 3, 6], 3));
        assert_eq!(1, Solution::smallest_range_ii(vec![7, 8, 8], 5));
    }
}
