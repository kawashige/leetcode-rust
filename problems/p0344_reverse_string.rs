pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        if n < 1 {
            return;
        }
        for i in 0..(n / 2) {
            s.swap(i, n - 1 - i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0344() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut input);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], input);

        let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H',], input);

        let mut input = vec!['a'];
        Solution::reverse_string(&mut input);
        assert_eq!(vec!['a'], input);
    }
}
