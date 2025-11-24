pub struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut count = [0; 26];
        let mut j = 0;
        let mut found = false;
        while j < s.len() {
            let i = (s.as_bytes()[j] - b'a') as usize;
            count[i] += 1;
            j += 1;
            if count[i] == k {
                found = true;
                break;
            }
        }
        if !found {
            return 0;
        }
        let mut result = s.len() + 1 - j;
        for i in 1..s.len() {
            let l = (s.as_bytes()[i - 1] - b'a') as usize;
            count[l] -= 1;
            if count[l] + 1 == k {
                let mut found = false;
                while j < s.len() {
                    let l = (s.as_bytes()[j] - b'a') as usize;
                    count[l] += 1;
                    j += 1;
                    if count[l] == k {
                        found = true;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            result += s.len() + 1 - j;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3325() {
        assert_eq!(Solution::number_of_substrings("abacb".to_string(), 2), 4);
        assert_eq!(Solution::number_of_substrings("abcde".to_string(), 1), 15);
    }
}
