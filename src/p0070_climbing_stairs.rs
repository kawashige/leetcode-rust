use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(0, 1);
        memo.insert(1, 1);

        fn count(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            match memo.get(&n) {
                Some(count) => count.clone(),
                None => {
                    let count = count(n - 1, memo) + count(n - 2, memo);
                    memo.insert(n, count);
                    count
                }
            }
        }

        count(n, &mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_70() {
        assert_eq!(2, Solution::climb_stairs(2));
    }
}
