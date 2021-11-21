pub struct Solution {}

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries
            .into_iter()
            .map(|q| {
                let q_bytes = q.as_bytes();
                let mut i = 0;
                for b in pattern.as_bytes() {
                    while i < q_bytes.len() && b != &q_bytes[i] {
                        if q_bytes[i].is_ascii_uppercase() {
                            return false;
                        }
                        i += 1;
                    }

                    if q_bytes.len() <= i {
                        return false;
                    }

                    i += 1;
                }

                (i..q.as_bytes().len()).all(|j| q_bytes[j].is_ascii_lowercase())
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1023() {
        assert_eq!(
            Solution::camel_match(
                vec![
                    "FooBar".to_string(),
                    "FooBarTest".to_string(),
                    "FootBall".to_string(),
                    "FrameBuffer".to_string(),
                    "ForceFeedBack".to_string()
                ],
                "FB".to_string()
            ),
            vec![true, false, true, true, false]
        );
        assert_eq!(
            Solution::camel_match(
                vec![
                    "FooBar".to_string(),
                    "FooBarTest".to_string(),
                    "FootBall".to_string(),
                    "FrameBuffer".to_string(),
                    "ForceFeedBack".to_string()
                ],
                "FoBa".to_string()
            ),
            vec![true, false, true, false, false]
        );
        assert_eq!(
            Solution::camel_match(
                vec![
                    "FooBar".to_string(),
                    "FooBarTest".to_string(),
                    "FootBall".to_string(),
                    "FrameBuffer".to_string(),
                    "ForceFeedBack".to_string()
                ],
                "FoBaT".to_string()
            ),
            vec![false, true, false, false, false]
        );
    }
}
