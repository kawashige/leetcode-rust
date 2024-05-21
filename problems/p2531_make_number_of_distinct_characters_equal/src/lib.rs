pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [usize; 26] {
        s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        })
    }

    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let counts1 = Self::count(&word1);
        let counts2 = Self::count(&word2);
        let count1 = counts1.iter().filter(|c| &&0 < c).count() as i32;
        let count2 = counts2.iter().filter(|c| &&0 < c).count() as i32;

        for i in 0..counts1.len() {
            for j in 0..counts2.len() {
                if 0 < counts1[i] && 0 < counts2[j] {
                    let mut after_count1 = count1;
                    let mut after_count2 = count2;
                    if i != j {
                        after_count1 += if counts1[j] == 0 { 1 } else { 0 }
                            - if counts1[i] == 1 { 1 } else { 0 };
                        after_count2 += if counts2[i] == 0 { 1 } else { 0 }
                            - if counts2[j] == 1 { 1 } else { 0 };
                    }
                    if after_count1 == after_count2 {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2531() {
        assert!(!Solution::is_it_possible(
            "aa".to_string(),
            "ab".to_string()
        ));
        assert!(!Solution::is_it_possible("ac".to_string(), "b".to_string()));
        assert!(Solution::is_it_possible(
            "abcc".to_string(),
            "aab".to_string()
        ));
        assert!(Solution::is_it_possible(
            "abcde".to_string(),
            "fghij".to_string()
        ));
    }
}
