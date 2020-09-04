pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        use std::collections::HashMap;
        let mut indices_map = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            indices_map.entry(c).or_insert(Vec::new()).push(i);
        }
        let mut indices = indices_map
            .values()
            .map(|v| (v[0], v.last().unwrap()))
            .collect::<Vec<(_, _)>>();

        indices.sort_by_key(|x| x.0);

        let mut results = Vec::new();
        let mut s = indices[0].0;
        let mut e = indices[0].1;
        for i in 1..indices.len() {
            if indices[i].0 <= *e {
                e = std::cmp::max(e, indices[i].1);
            } else {
                results.push((e - s + 1) as i32);
                s = indices[i].0;
                e = indices[i].1;
            }
        }
        results.push((e - s + 1) as i32);
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day4() {
        assert_eq!(
            vec![9, 7, 8],
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
        );
        assert_eq!(vec![1], Solution::partition_labels("a".to_string()));
    }
}
