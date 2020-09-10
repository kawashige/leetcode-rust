pub struct Solution {}

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;

        let mut cows_s = Vec::new();
        let mut cows_g = Vec::new();
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                a += 1;
            } else {
                cows_s.push(s);
                cows_g.push(g);
            }
        }

        cows_s.sort();
        cows_g.sort();
        let mut i = 0;
        let mut j = 0;
        while i < cows_s.len() && j < cows_g.len() {
            if cows_s[i] == cows_g[j] {
                b += 1;
                i += 1;
                j += 1;
            } else if cows_s[i] < cows_g[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        format!("{}A{}B", a, b)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(
            "1A3B".to_string(),
            Solution::get_hint("1807".to_string(), "7810".to_string())
        );
        assert_eq!(
            "1A1B".to_string(),
            Solution::get_hint("1123".to_string(), "0111".to_string())
        );
    }
}
