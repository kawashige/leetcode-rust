pub struct Solution {}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .map(|(i, s)| {
                if i % 2 == 1 {
                    0
                } else {
                    s.as_bytes().iter().filter(|b| b == &&b'*').count()
                }
            })
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2315() {
        assert_eq!(
            Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()),
            2
        );
        assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
        assert_eq!(
            Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()),
            5
        );
    }
}
