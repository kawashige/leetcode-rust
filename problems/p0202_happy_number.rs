pub struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;

        fn convert(n: i32) -> i32 {
            n.to_string()
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap().pow(2))
                .sum()
        }

        let mut history = HashSet::new();
        let mut current = n;
        while !history.contains(&current) {
            history.insert(current);
            current = convert(current);
            if current == 1 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0202() {
        assert!(Solution::is_happy(19));
    }
}
