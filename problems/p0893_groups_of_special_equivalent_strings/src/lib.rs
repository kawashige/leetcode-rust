use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        words
            .into_iter()
            .fold(HashMap::new(), |mut map, word| {
                let (mut even, mut odd) =
                    word.chars()
                        .fold((Vec::new(), Vec::new()), |(mut even, mut odd), c| {
                            if even.len() == odd.len() {
                                even.push(c);
                            } else {
                                odd.push(c);
                            }
                            (even, odd)
                        });

                even.sort_unstable();
                odd.sort_unstable();

                *map.entry((
                    even.into_iter().collect::<String>(),
                    odd.into_iter().collect::<String>(),
                ))
                .or_insert(0) += 1;
                map
            })
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0893() {
        assert_eq!(
            Solution::num_special_equiv_groups(vec![
                "abcd".to_string(),
                "cdab".to_string(),
                "cbad".to_string(),
                "xyzz".to_string(),
                "zzxy".to_string(),
                "zzyx".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::num_special_equiv_groups(vec![
                "abc".to_string(),
                "acb".to_string(),
                "bac".to_string(),
                "bca".to_string(),
                "cab".to_string(),
                "cba".to_string()
            ]),
            3
        );
    }
}
