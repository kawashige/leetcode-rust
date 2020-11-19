pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut r = ransom_note.chars().collect::<Vec<char>>();
        let mut m = magazine.chars().collect::<Vec<char>>();
        r.sort();
        m.sort();

        let mut i = 0;
        let mut j = 0;
        while i < r.len() && j < m.len() {
            if r[i] == m[j] {
                i += 1;
                j += 1;
            } else if m[j] < r[i] {
                j += 1;
            } else {
                break;
            }
        }

        println!("i: {}, j: {}", i, j);

        i == r.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0383() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}
