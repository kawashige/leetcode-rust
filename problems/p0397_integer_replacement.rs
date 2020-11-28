pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        fn recurse(n: i64, memo: &mut HashMap<i64, i32>) -> i32 {
            if n == 1 {
                return 0;
            } else if memo.contains_key(&n) {
                return memo[&n];
            }

            let result = if n % 2 == 0 {
                recurse(n / 2, memo)
            } else {
                std::cmp::min(recurse(n + 1, memo), recurse(n - 1, memo))
            } + 1;
            memo.insert(n, result);
            println!("memo: {:?}", memo);
            result
        }
        recurse(n as i64, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0397() {
        assert_eq!(32, Solution::integer_replacement(2147483647));
        assert_eq!(10, Solution::integer_replacement(1024));
        assert_eq!(11, Solution::integer_replacement(1025));
        assert_eq!(10, Solution::integer_replacement(1024));
        assert_eq!(31, Solution::integer_replacement(100000000));
        assert_eq!(3, Solution::integer_replacement(8));
        assert_eq!(4, Solution::integer_replacement(7));
        assert_eq!(2, Solution::integer_replacement(4));
        assert_eq!(1, Solution::integer_replacement(2));
        assert_eq!(0, Solution::integer_replacement(1));
    }
}
