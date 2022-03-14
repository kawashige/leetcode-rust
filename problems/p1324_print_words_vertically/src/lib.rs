pub struct Solution {}

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        s.split_ascii_whitespace()
            .enumerate()
            .fold(Vec::new(), |mut v, (i, w)| {
                for (j, c) in w.chars().enumerate() {
                    if v.len() == j {
                        v.push(String::new());
                    }
                    while v[j].len() < i {
                        v[j].push(' ');
                    }
                    v[j].push(c);
                }
                v
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1324() {
        assert_eq!(
            Solution::print_vertically("HOW ARE YOU".to_string()),
            ["HAY", "ORO", "WEU"]
        );
        assert_eq!(
            Solution::print_vertically("TO BE OR NOT TO BE".to_string()),
            ["TBONTB", "OEROOE", "   T"]
        );
        assert_eq!(
            Solution::print_vertically("CONTEST IS COMING".to_string()),
            ["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]
        );
    }
}
