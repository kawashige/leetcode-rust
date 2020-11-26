pub struct Solution {}

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut len = vec![0; s.len()];
        let mut indexes = vec![Vec::new(); 26];

        for (i, c) in s.chars().enumerate() {
            indexes[c as usize - 0x61].push(i);
            if k as usize <= i + 1 {
                if let Some(j) = indexes
                    .iter()
                    .filter(|v| !v.is_empty() && v.len() < k as usize)
                    .map(|v| v.last().unwrap())
                    .max()
                {
                    if indexes.iter().all(|v| {
                        let count = v.iter().filter(|vv| j < *vv).count();
                        count == 0 || k as usize <= count
                    }) {
                        len[i] = i - j;
                    }
                } else {
                    len[i] = i + 1;
                }
            }
        }
        println!("len: {:?}, indexes: {:?}", len, indexes);

        len.into_iter().max().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day26() {
        assert_eq!(3, Solution::longest_substring("aaabb".to_string(), 3));
        assert_eq!(5, Solution::longest_substring("ababbc".to_string(), 2));
        assert_eq!(4, Solution::longest_substring("abacbbaa".to_string(), 2));
        assert_eq!(0, Solution::longest_substring("a".to_string(), 2));
    }
}
