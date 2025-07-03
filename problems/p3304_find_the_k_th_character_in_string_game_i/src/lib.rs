pub struct Solution {}

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut chars = vec![0];
        while chars.len() < k as usize {
            let l = chars.len();
            for i in 0..l {
                chars.push((chars[i] + 1) % 26);
            }
        }
        (chars[k as usize - 1] + b'a') as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3304() {
        assert_eq!(Solution::kth_character(5), 'b');
        assert_eq!(Solution::kth_character(10), 'c');
    }
}
