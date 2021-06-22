pub struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        s.chars()
            .rev()
            .zip(shifts.into_iter().rev())
            .fold((Vec::new(), 0), |(mut s, i), (c, j)| {
                let k = ((i + j) % 26) as u8;
                s.push((0x61 + (c as u8 - 0x61 + k) % 26) as char);
                (s, k as i32)
            })
            .0
            .into_iter()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0848() {
        assert_eq!(
            Solution::shifting_letters("ruu".to_string(), vec![26, 9, 17]),
            "rul".to_string()
        );
        assert_eq!(
            Solution::shifting_letters("abc".to_string(), vec![3, 5, 9]),
            "rpl".to_string()
        );
    }
}
