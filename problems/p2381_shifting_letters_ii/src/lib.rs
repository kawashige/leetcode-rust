pub struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut total_shift = vec![0; s.len() + 1];
        for shift in shifts {
            total_shift[shift[0] as usize] += if shift[2] == 0 { -1 } else { 1 };
            total_shift[shift[1] as usize + 1] += if shift[2] == 0 { 1 } else { -1 };
        }

        let mut result = String::new();
        for i in 0..s.len() {
            if 0 < i {
                total_shift[i] += total_shift[i - 1];
            }
            let c = b'a'
                + (((((s.as_bytes()[i] - b'a') as i32 + total_shift[i]) % 26) + 26) % 26) as u8;
            result.push(c as char);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2381() {
        assert_eq!(
            Solution::shifting_letters(
                "abc".to_string(),
                vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
            ),
            "ace".to_string()
        );
        assert_eq!(
            Solution::shifting_letters("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]]),
            "catz".to_string()
        );
    }
}
