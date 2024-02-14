pub struct Solution {}

impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        for b in 2..n - 1 {
            let mut digits = Vec::new();

            let mut remains = n;
            let mut div = 1;
            while div * b <= n {
                div *= b;
            }

            while 0 < div {
                if 0 < remains / div {
                    digits.push(remains / div);
                    remains = remains % div;
                } else {
                    digits.push(0);
                }
                div /= b;
            }

            if (0..digits.len()).any(|i| digits[i] != digits[digits.len() - 1 - i]) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2396() {
        assert!(!Solution::is_strictly_palindromic(9));
        assert!(!Solution::is_strictly_palindromic(4));
        assert!(!Solution::is_strictly_palindromic(100000));
    }
}
