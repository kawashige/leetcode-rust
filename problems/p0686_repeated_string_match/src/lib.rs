pub struct Solution {}

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if b.is_empty() {
            return 0;
        }

        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();

        let mut i = 0;
        let mut j = 0;
        while j < bytes_b.len() {
            while i < bytes_a.len() && bytes_a[i] != bytes_b[j] {
                i += 1;
            }

            if i == bytes_a.len() {
                break;
            }

            let prev_i = i;
            let prev_j = j;

            while j < bytes_b.len() && bytes_a[i] == bytes_b[j] {
                i = (i + 1) % bytes_a.len();
                j += 1;
            }

            if j == bytes_b.len() {
                return (1 + (bytes_b.len() + prev_i - 1) / bytes_a.len()) as i32;
            }

            i = prev_i + 1;
            j = prev_j;
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0686() {
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "".to_string()),
            0
        );
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
            3
        );
        assert_eq!(
            Solution::repeated_string_match("a".to_string(), "aa".to_string()),
            2
        );
        assert_eq!(
            Solution::repeated_string_match("aa".to_string(), "a".to_string()),
            1
        );
        assert_eq!(
            Solution::repeated_string_match("abc".to_string(), "wxyz".to_string()),
            -1
        );
    }
}
