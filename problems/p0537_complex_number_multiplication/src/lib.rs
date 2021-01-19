pub struct Solution {}

impl Solution {
    pub fn complex_number_multiply(a: String, b: String) -> String {
        fn parse_complex(n: String) -> (i32, i32) {
            let mut splitted = n.trim_end_matches("i").split("+");
            let a = splitted.next().unwrap().parse::<i32>().unwrap();
            let b = splitted.next().unwrap().parse::<i32>().unwrap();
            (a, b)
        }

        let (a1, a2) = parse_complex(a);
        let (b1, b2) = parse_complex(b);
        format!("{}+{}i", a1 * b1 - a2 * b2, a2 * b1 + a1 * b2)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0537() {
        assert_eq!(
            "0+2i".to_string(),
            Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string())
        );
        assert_eq!(
            "0+-2i".to_string(),
            Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string())
        );
    }
}
