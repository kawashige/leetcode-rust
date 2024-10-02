use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        cur: usize,
        remains: &mut Vec<i32>,
        len: i32,
        memo: &mut HashMap<String, i32>,
    ) -> i32 {
        let key = format!("{},{},{}", remains[0], remains[1], remains[2]);
        if let Some(max_len) = memo.get(&key) {
            return *max_len;
        }

        let mut max_len = len;
        let next = [vec![1], vec![0, 2], vec![0, 2]];
        for i in &next[cur] {
            if 0 < remains[*i] {
                remains[*i] -= 1;
                max_len = max_len.max(Self::recurse(*i, remains, len + 2, memo));
                remains[*i] += 1;
            }
        }

        memo.insert(key, max_len);
        max_len
    }

    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        let mut len = 0;
        let mut remains = vec![x, y, z];
        for i in 0..remains.len() {
            if 0 < remains[i] {
                remains[i] -= 1;
                len = len.max(Self::recurse(i, &mut remains, 2, &mut HashMap::new()));
                remains[i] += 1;
            }
        }
        len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2745() {
        assert_eq!(Solution::longest_string(50, 50, 50), 12);
        assert_eq!(Solution::longest_string(2, 5, 1), 12);
        assert_eq!(Solution::longest_string(3, 2, 2), 14);
    }
}
