pub struct Solution {}

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut count = num.as_bytes().iter().fold([0; 10], |mut count, b| {
            count[(b - b'0') as usize] += 1;
            count
        });

        let mut s = String::new();

        if let Some(d) = (1..10).rev().find(|i| 1 < count[*i]) {
            s.push((d as u8 + b'0') as char);
            count[d] -= 2;
            while let Some(d) = (0..10).rev().find(|i| 1 < count[*i]) {
                s.push((d as u8 + b'0') as char);
                count[d] -= 2;
            }
        }

        if let Some(d) = (0..10).rev().find(|i| 0 < count[*i]) {
            format!(
                "{}{}{}",
                s,
                (d as u8 + b'0') as char,
                s.chars().rev().collect::<String>()
            )
        } else {
            format!("{}{}", s, s.chars().rev().collect::<String>())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2384() {
        assert_eq!(
            Solution::largest_palindromic("444947137".to_string()),
            "7449447".to_string()
        );
        assert_eq!(
            Solution::largest_palindromic("00009".to_string()),
            "9".to_string()
        );
    }
}
