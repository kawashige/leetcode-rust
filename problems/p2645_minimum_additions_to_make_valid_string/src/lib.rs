pub struct Solution {}

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let bytes = [b'a', b'b', b'c'];
        let mut i = 0;
        let mut count = 0;
        for j in 0..word.len() {
            while word.as_bytes()[j] != bytes[i] {
                count += 1;
                i = (i + 1) % bytes.len();
            }
            i = (i + 1) % bytes.len();
        }
        println!("count: {}", count);
        count + (bytes.len() - 1 - (i + bytes.len() - 1) % bytes.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2645() {
        assert_eq!(Solution::add_minimum("b".to_string()), 2);
        assert_eq!(Solution::add_minimum("aaa".to_string()), 6);
        assert_eq!(Solution::add_minimum("abc".to_string()), 0);
    }
}
