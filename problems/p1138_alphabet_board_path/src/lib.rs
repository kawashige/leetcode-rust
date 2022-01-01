pub struct Solution {}

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let (mut i, mut j) = (0, 0);
        let mut s = String::new();

        for c in target.chars() {
            let new_i = (c as i32 - 0x61) / 5;
            let new_j = (c as i32 - 0x61) % 5;

            if c == 'z' {
                for _ in 0..(j - new_j).abs() {
                    s.push(if j < new_j { 'R' } else { 'L' });
                }
                for _ in 0..(i - new_i).abs() {
                    s.push(if i < new_i { 'D' } else { 'U' });
                }
            } else {
                for _ in 0..(i - new_i).abs() {
                    s.push(if i < new_i { 'D' } else { 'U' });
                }
                for _ in 0..(j - new_j).abs() {
                    s.push(if j < new_j { 'R' } else { 'L' });
                }
            }
            s.push('!');
            i = new_i;
            j = new_j;
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1138() {
        assert_eq!(
            Solution::alphabet_board_path("leet".to_string()),
            "DDR!UURRR!!DDD!".to_string()
        );
        assert_eq!(
            Solution::alphabet_board_path("code".to_string()),
            "RR!DDRR!UUL!R!".to_string()
        );
    }
}
