use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut c = 0;
        let mut queue = VecDeque::new();
        for i in 0..a.len().max(b.len()) {
            let b = if a.len() <= i {
                0
            } else {
                (a.as_bytes()[a.len() - 1 - i] - b'0') as i32
            } + if b.len() <= i {
                0
            } else {
                (b.as_bytes()[b.len() - 1 - i] - b'0') as i32
            } + c;
            c = b / 2;
            queue.push_front(if b % 2 == 0 { '0' } else { '1' });
        }
        if c == 1 {
            queue.push_front('1');
        }
        queue.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_67() {
        let ret = Solution::add_binary("1010".to_string(), "1011".to_string());
        assert_eq!("10101", ret);
    }
}
