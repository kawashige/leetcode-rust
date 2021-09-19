pub struct Solution {}

impl Solution {
    pub fn recurse(
        num: &str,
        val: i64,
        last: i64,
        target: i64,
        eval: &mut Vec<String>,
        result: &mut Vec<String>,
    ) {
        if num.is_empty() {
            if val == target {
                result.push(eval.join(""));
            }
            return;
        }

        for j in 0..if num.starts_with("0") { 1 } else { num.len() } {
            let x = num[..=j].parse::<i64>().unwrap();

            if !eval.is_empty() {
                eval.push("+".to_string());
            }
            eval.push(x.to_string());
            Self::recurse(&num[(j + 1)..], val + x, x, target, eval, result);
            eval.pop();
            eval.pop();

            if !eval.is_empty() {
                eval.push("-".to_string());
                eval.push(x.to_string());
                Self::recurse(&num[(j + 1)..], val - x, -x, target, eval, result);
                eval.pop();
                eval.pop();

                eval.push("*".to_string());
                eval.push(x.to_string());
                Self::recurse(
                    &num[(j + 1)..],
                    val - last + last * x,
                    last * x,
                    target,
                    eval,
                    result,
                );
                eval.pop();
                eval.pop();
            }
        }
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();

        Self::recurse(
            num.as_str(),
            0,
            0,
            target as i64,
            &mut Vec::new(),
            &mut result,
        );

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day19() {
        assert_eq!(
            Solution::add_operators("123".to_string(), 6),
            vec!["1+2+3".to_string(), "1*2*3".to_string()]
        );
        assert_eq!(
            Solution::add_operators("232".to_string(), 8),
            vec!["2+3*2".to_string(), "2*3+2".to_string()]
        );
        assert_eq!(
            Solution::add_operators("105".to_string(), 5),
            vec!["1*0+5".to_string(), "10-5".to_string()]
        );
        assert_eq!(
            Solution::add_operators("00".to_string(), 0),
            vec!["0+0".to_string(), "0-0".to_string(), "0*0".to_string()]
        );
        assert_eq!(
            Solution::add_operators("3456237490".to_string(), 9191),
            vec![] as Vec<String>
        );
    }
}
