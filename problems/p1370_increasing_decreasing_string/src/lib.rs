pub struct Solution {}

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[*b as usize - 0x61] += 1;
            count
        });

        let mut result = String::new();
        let mut d = 1;
        let mut i = 0;

        while result.len() < s.len() {
            if 0 < count[i] {
                count[i] -= 1;
                result.push((i as u8 + 0x61) as char);
            }
            if i == 0 && d == -1 {
                d = 1;
            } else if i == count.len() - 1 && d == 1 {
                d = -1;
            } else {
                i = (i as i32 + d) as usize;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1370() {
        assert_eq!(
            Solution::sort_string("aaaabbbbcccc".to_string()),
            "abccbaabccba".to_string()
        );
        assert_eq!(Solution::sort_string("rat".to_string()), "art".to_string());
    }
}
