pub struct Solution {}

impl Solution {
    pub fn is_ideal_permutation(mut a: Vec<i32>) -> bool {
        for i in 0..a.len() {
            if a[i] as usize == i + 1 && a[i + 1] as usize == i {
                a.swap(i, i + 1);
            } else if a[i] as usize != i {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0775() {
        assert!(Solution::is_ideal_permutation(vec![0, 2, 1]));
        assert!(Solution::is_ideal_permutation(vec![1, 0, 2]));
        assert!(!Solution::is_ideal_permutation(vec![1, 2, 0]));
    }
}
