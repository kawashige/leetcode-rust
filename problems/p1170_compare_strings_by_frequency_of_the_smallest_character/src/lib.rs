pub struct Solution {}

impl Solution {
    pub fn f(s: &str) -> usize {
        *s.chars()
            .fold([0; 26], |mut count, c| {
                count[c as usize - 0x61] += 1;
                count
            })
            .iter()
            .find(|count| count > &&0)
            .unwrap()
    }

    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let n = words.len() as i32;
        let mut acc_count = words.into_iter().fold([0; 2001], |mut acc, w| {
            acc[Self::f(&w)] += 1;
            acc
        });

        for i in 1..acc_count.len() {
            acc_count[i] += acc_count[i - 1];
        }

        queries
            .into_iter()
            .map(|q| n - acc_count[Self::f(&q)])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1170() {
        assert_eq!(
            Solution::num_smaller_by_frequency(vec!["cbd".to_string()], vec!["zaaaz".to_string()]),
            vec![1]
        );
        assert_eq!(
            Solution::num_smaller_by_frequency(
                vec!["bbb".to_string(), "cc".to_string()],
                vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string()
                ]
            ),
            vec![1, 2]
        );
    }
}
