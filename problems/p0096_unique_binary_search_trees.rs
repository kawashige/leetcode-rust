pub struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        use std::collections::HashMap;

        fn count(nums: &[i32], memo: &mut HashMap<String, i32>) -> i32 {
            match nums.len() {
                0 => 1,
                _ => {
                    let key = format!("{},{}", nums[0], nums[nums.len() - 1]);
                    match memo.get(&key) {
                        Some(n) => *n,
                        None => {
                            let n = (0..nums.len()).fold(0, |acc, i| {
                                acc + count(&nums[..i], memo) * count(&nums[(i + 1)..], memo)
                            });
                            memo.insert(key, n);
                            n
                        }
                    }
                }
            }
        }

        count(&(1..=n).collect::<Vec<i32>>(), &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0096() {
        assert_eq!(5, Solution::num_trees(3));
        assert_eq!(1, Solution::num_trees(1));
        assert_eq!(2, Solution::num_trees(2));
        assert_eq!(1767263190, Solution::num_trees(19));
    }
}
