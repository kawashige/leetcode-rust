pub struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        fn recurse(n: i32, k: i32) -> i32 {
            if n == 0 {
                0
            } else if k % 2 == 0 {
                recurse(n - 1, k / 2)
            } else {
                [1, 0][recurse(n - 1, k / 2) as usize]
            }
        }
        recurse(n - 1, k - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0779() {
        assert_eq!(Solution::kth_grammar(3, 1), 0);
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(4, 5), 1);
    }
}
