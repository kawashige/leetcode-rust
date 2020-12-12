pub struct Solution {}

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut char_count = [0; 26];
        let mut result = [0; 10];

        for c in s.chars() {
            char_count[c as usize - 0x61] += 1;
        }

        for (c, n, i) in [
            ('z', "zero".to_string(), 0usize),
            ('x', "six".to_string(), 6),
            ('w', "two".to_string(), 2),
            ('g', "eight".to_string(), 8),
            ('t', "three".to_string(), 3),
            ('r', "four".to_string(), 4),
            ('f', "five".to_string(), 5),
            ('v', "seven".to_string(), 7),
            ('i', "nine".to_string(), 9),
            ('o', "one".to_string(), 1),
        ]
        .iter()
        {
            let count = char_count[*c as usize - 0x61];
            if 0 < count {
                for n_c in n.chars() {
                    char_count[n_c as usize - 0x61] -= count;
                }
                result[*i] += count;
            }
        }

        println!("char_count: {:?}, result: {:?}", char_count, result);

        (0..=9)
            .map(|i| i.to_string().repeat(result[i]))
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0423() {
        assert_eq!(
            "012".to_string(),
            Solution::original_digits("owoztneoer".to_string())
        );
        assert_eq!(
            "45".to_string(),
            Solution::original_digits("fviefuro".to_string())
        );
        assert_eq!(
            "11".to_string(),
            Solution::original_digits("oneone".to_string())
        );
        assert_eq!(
            "9".to_string(),
            Solution::original_digits("nine".to_string())
        );
    }
}
