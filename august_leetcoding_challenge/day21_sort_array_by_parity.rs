pub struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<i32>, Vec<i32>) = a.iter().partition(|x| *x % 2 == 0);
        even.append(&mut odd);
        even
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day21() {
        assert_eq!(
            vec![2, 4, 3, 1],
            Solution::sort_array_by_parity(vec![3, 1, 2, 4])
        );
        assert_eq!(vec![2, 4], Solution::sort_array_by_parity(vec![2, 4]));
        assert_eq!(vec![1, 3], Solution::sort_array_by_parity(vec![1, 3]));
    }
}
