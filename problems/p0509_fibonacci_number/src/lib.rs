use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        fn recurse(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if let Some(v) = memo.get(&n) {
                *v
            } else if n < 2 {
                n
            } else if n == 2 {
                1
            } else {
                let result = recurse(n - 1, memo) + recurse(n - 2, memo);
                memo.insert(n, result);
                result
            }
        }

        recurse(n, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0509() {
        assert_eq!(1, Solution::fib(2));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
    }
}
