pub struct Solution {}

impl Solution {
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let mut arr = vec![1; n as usize];
        for _ in 0..k {
            for i in 1..arr.len() {
                arr[i] = (arr[i] + arr[i - 1]) % M;
            }
        }
        *arr.last().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3179() {
        assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
        assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
    }
}
