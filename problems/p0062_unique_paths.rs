pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        use std::collections::HashMap;

        fn find_paths(m: i32, n: i32, memo: &mut HashMap<String, i32>) -> i32 {
            if m == 1 && n == 1 {
                return 1;
            }

            let key = format!("{},{}", m, n);
            match memo.get(&key) {
                Some(n) => *n,
                None => {
                    let mut paths = 0;
                    if (m - 1) > 0 {
                        paths += find_paths(m - 1, n, memo);
                    }
                    if (n - 1) > 0 {
                        paths += find_paths(m, n - 1, memo);
                    }
                    memo.insert(key, paths);
                    paths
                }
            }
        }

        find_paths(m, n, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0062() {
        assert_eq!(3, Solution::unique_paths(3, 2));
        assert_eq!(1, Solution::unique_paths(1, 1));
    }
}
