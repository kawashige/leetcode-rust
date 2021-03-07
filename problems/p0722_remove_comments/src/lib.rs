pub struct Solution {}

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let source_str = source.join("\n");
        let mut chars = source_str.chars().peekable();

        println!("{}", source_str);

        while let Some(c) = chars.next() {
            println!("{}", c);
            match c {
                '/' if chars.peek() == Some(&'/') => {
                    while chars.peek() != Some(&'\n') && chars.peek() != None {
                        chars.next();
                    }
                }
                '/' if chars.peek() == Some(&'*') => {
                    chars.next();
                    let mut cc = chars.next();
                    while !(cc == Some('*') && chars.peek() == Some(&'/')) {
                        cc = chars.next();
                    }
                    chars.next();
                }
                _ => result.push(c),
            }
        }

        result
            .into_iter()
            .collect::<String>()
            .split("\n")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0722() {
        assert_eq!(
            Solution::remove_comments(vec![
                "/*Test program */".to_string(),
                "int main()".to_string(),
                "{ ".to_string(),
                "  // variable declaration ".to_string(),
                "int a, b, c;".to_string(),
                "/* This is a test".to_string(),
                "   multiline  ".to_string(),
                "   comment for ".to_string(),
                "   testing */".to_string(),
                "a = b + c;".to_string(),
                "}".to_string()
            ]),
            vec![
                "int main()".to_string(),
                "{ ".to_string(),
                "  ".to_string(),
                "int a, b, c;".to_string(),
                "a = b + c;".to_string(),
                "}".to_string()
            ]
        );

        assert_eq!(
            Solution::remove_comments(vec![
                "a/*comment".to_string(),
                "line".to_string(),
                "more_comment*/b".to_string()
            ]),
            vec!["ab".to_string()]
        );

        assert_eq!(
            Solution::remove_comments(vec![
                "a//*b//*c".to_string(),
                "blank".to_string(),
                "d//*e/*/f".to_string()
            ]),
            vec!["a".to_string(), "blank".to_string(), "d".to_string()]
        );
    }
}
