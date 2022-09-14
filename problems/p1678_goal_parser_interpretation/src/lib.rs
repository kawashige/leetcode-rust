pub struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let bytes = command.as_bytes();
        let mut s = String::new();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'G' {
                s.push('G');
            } else if bytes[i + 1] == b')' {
                s.push('o');
                i += 1;
            } else {
                s += "al";
                i += 3;
            }
            i += 1;
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1678() {
        assert_eq!(
            Solution::interpret("G()(al)".to_string()),
            "Goal".to_string()
        );
        assert_eq!(
            Solution::interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
        assert_eq!(
            Solution::interpret("(al)G(al)()()G".to_string()),
            "alGalooG".to_string()
        );
    }
}
