use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut result = 1;
        let mut c = 1;
        for i in 1..s.len() {
            if s.as_bytes()[i] == s.as_bytes()[i - 1] {
                c += 1;
            } else {
                result = result.max(c);
                c = 1;
            }
        }
        result = result.max(c);

        for c in 'a'..='c' {
            let mut map = HashMap::new();
            map.insert(0, -1);
            let (x, y) = match c {
                'a' => (1, 2),
                'b' => (0, 2),
                _ => (0, 1),
            };
            let mut count = [0; 3];
            for i in 0..s.len() {
                if s.as_bytes()[i] == c as u8 {
                    map.clear();
                    map.insert(0, i as i32);
                    count = [0; 3];
                } else {
                    count[(s.as_bytes()[i] - b'a') as usize] += 1;
                    if let Some(j) = map.get(&(count[y] - count[x])) {
                        result = result.max(i as i32 - j);
                    } else {
                        map.insert(count[y] - count[x], i as i32);
                    }
                }
            }
        }

        let mut map = HashMap::new();
        map.insert((0, 0), -1);
        let mut count = [0; 3];

        for i in 0..s.len() {
            count[(s.as_bytes()[i] - b'a') as usize] += 1;
            if let Some(j) = map.get(&(count[1] - count[0], count[2] - count[0])) {
                result = result.max(i as i32 - j);
            } else {
                map.insert((count[1] - count[0], count[2] - count[0]), i as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3714() {
        assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
        assert_eq!(Solution::longest_balanced("aabcc".to_string()), 3);
        assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
    }
}
