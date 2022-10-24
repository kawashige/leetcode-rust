pub struct Solution {}

impl Solution {
    pub fn count_character(s: &str) -> [usize; 26] {
        s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        })
    }

    pub fn min_characters(a: String, b: String) -> i32 {
        let count_a = Self::count_character(&a);
        let count_b = Self::count_character(&b);

        let mut condition1 = std::usize::MAX;
        let mut a_count = 0;
        let mut b_count = 0;
        for i in 0..25 {
            a_count += count_a[i];
            b_count += count_b[i];

            condition1 = condition1.min(a.len() - a_count + b_count);
        }

        let mut condition2 = std::usize::MAX;
        let mut a_count = 0;
        let mut b_count = 0;
        for i in 0..25 {
            a_count += count_a[i];
            b_count += count_b[i];

            condition2 = condition2.min(b.len() - b_count + a_count);
        }

        let condition3 =
            a.len() - count_a.iter().max().unwrap() + b.len() - count_b.iter().max().unwrap();

        condition1.min(condition2).min(condition3) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1737() {
        assert_eq!(
            Solution::min_characters(
                "a".to_string(),
                "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::min_characters("aba".to_string(), "caa".to_string()),
            2
        );
        assert_eq!(
            Solution::min_characters("dabadd".to_string(), "cda".to_string()),
            3
        );
    }
}
