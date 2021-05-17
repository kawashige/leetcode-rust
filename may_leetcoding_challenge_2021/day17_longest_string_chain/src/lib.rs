use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        words
            .into_iter()
            .fold(vec![HashMap::new(); 17], |mut group, w| {
                group[w.len()].insert(w, 1);
                group
            })
            .into_iter()
            .rev()
            .fold(
                (1, HashMap::<String, i32>::new()),
                |(mut max, prev), mut group| {
                    for (k, v) in &prev {
                        let k = k.as_bytes();
                        (0..k.len()).for_each(|i| {
                            let s = (0..i)
                                .chain((i + 1)..k.len())
                                .map(|j| k[j] as char)
                                .collect::<String>();
                            if let Some(c) = group.get_mut(&s) {
                                *c = std::cmp::max(*c, v + 1);
                                max = std::cmp::max(max, *c);
                            }
                        });
                    }
                    (max, group)
                },
            )
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day17() {
        assert_eq!(
            Solution::longest_str_chain(vec![
                "a".to_string(),
                "b".to_string(),
                "ba".to_string(),
                "bca".to_string(),
                "bda".to_string(),
                "bdca".to_string()
            ]),
            4
        );
        assert_eq!(
            Solution::longest_str_chain(vec![
                "xbc".to_string(),
                "pcxbcf".to_string(),
                "xb".to_string(),
                "cxbc".to_string(),
                "pcxbc".to_string()
            ]),
            5
        );
    }
}
