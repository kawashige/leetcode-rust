pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        fn recurse(
            integers: i32,
            total: i32,
            is_first: bool,
            memo: &mut HashMap<(i32, i32, bool), bool>,
        ) -> bool {
            if let Some(v) = memo.get(&(integers, total, is_first)) {
                return *v;
            }
            if integers == 0 {
                return false;
            }
            if total as u32 <= 32 - integers.leading_zeros() - 1 {
                return is_first;
            }
            let mut results = (0..32)
                .filter(|i| 0 < integers & (1 << i))
                .map(|i| recurse(integers & !(1 << i), total - i, !is_first, memo));

            let result = if is_first {
                results.any(|x| x)
            } else {
                results.all(|x| x)
            };
            memo.insert((integers, total, is_first), result);

            result
        }

        recurse(
            (1..=max_choosable_integer).fold(0, |acc, i| acc | (1 << i)),
            desired_total,
            true,
            &mut HashMap::new(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0464() {
        assert!(!Solution::can_i_win(20, 210));
        assert!(!Solution::can_i_win(10, 11));
        assert!(Solution::can_i_win(10, 0));
        assert!(Solution::can_i_win(10, 1));
    }
}
