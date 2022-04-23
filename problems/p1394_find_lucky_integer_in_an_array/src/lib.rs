use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        arr.into_iter()
            .fold(BTreeMap::new(), |mut map, x| {
                *map.entry(x).or_insert(0) += 1;
                map
            })
            .into_iter()
            .rev()
            .find(|(x, c)| x == c)
            .unwrap_or((-1, -1))
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1394() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
        assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    }
}
