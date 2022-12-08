pub struct Solution {}

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let mut perm = (0..n as usize).collect::<Vec<_>>();
        let mut count = 0;

        while 0 == count || (0..perm.len()).any(|i| perm[i] != i) {
            let mut arr = vec![0; perm.len()];
            for i in (0..arr.len()).step_by(2) {
                arr[i] = perm[i / 2];
            }
            for i in (1..arr.len()).step_by(2) {
                arr[i] = perm[perm.len() / 2 + (i - 1) / 2];
            }
            count += 1;
            perm = arr;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1806() {
        assert_eq!(Solution::reinitialize_permutation(2), 1);
        assert_eq!(Solution::reinitialize_permutation(4), 2);
        assert_eq!(Solution::reinitialize_permutation(6), 4);
    }
}
