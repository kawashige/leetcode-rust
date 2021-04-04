pub struct Solution {}

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let mut start = start.chars().collect::<Vec<char>>();
        let end = end.chars().collect::<Vec<char>>();

        let mut i = 0;
        while i < start.len() {
            match (start[i], end[i]) {
                ('R', 'X') | ('X', 'L') => {
                    if let Some(j) = ((i + 1)..start.len())
                        .skip_while(|j| start[*j] == start[i])
                        .next()
                    {
                        if start[j] == end[i] {
                            start.swap(i, j);
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                ('X', 'X') | ('R', 'R') | ('L', 'L') => {}
                _ => {
                    return false;
                }
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0777() {
        assert!(Solution::can_transform(
            "XXXXXLXXXX".to_string(),
            "LXXXXXXXXX".to_string()
        ));
        assert!(!Solution::can_transform(
            "LXXLXRLXXL".to_string(),
            "XLLXRXLXLX".to_string()
        ));
        assert!(Solution::can_transform(
            "RXXLRXRXL".to_string(),
            "XRLXXRRLX".to_string()
        ));
        assert!(!Solution::can_transform("X".to_string(), "L".to_string()));
        assert!(!Solution::can_transform(
            "LLR".to_string(),
            "RRL".to_string()
        ));
        assert!(Solution::can_transform("XL".to_string(), "LX".to_string()));
        assert!(!Solution::can_transform(
            "XLLR".to_string(),
            "LXLX".to_string()
        ));
    }
}
