pub struct Solution {}

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else {
            Self::gcd(b % a, a)
        }
    }
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        (2..=n)
            .map(|i| {
                (1..i).flat_map(move |j| {
                    if Self::gcd(i, j) == 1 {
                        Some(format!("{}/{}", j, i))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1447() {
        assert_eq!(Solution::simplified_fractions(1), vec![] as Vec<String>);
        assert_eq!(Solution::simplified_fractions(2), vec!["1/2".to_string()]);
        assert_eq!(
            Solution::simplified_fractions(3),
            vec!["1/2".to_string(), "1/3".to_string(), "2/3".to_string()]
        );
        assert_eq!(
            Solution::simplified_fractions(4),
            vec![
                "1/2".to_string(),
                "1/3".to_string(),
                "2/3".to_string(),
                "1/4".to_string(),
                "3/4".to_string()
            ]
        );
    }
}
