use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut count, (i, n)| {
                let entry = count.entry(n).or_insert((0, 50000, 0));
                entry.0 += 1;
                entry.1 = std::cmp::min(entry.1, i);
                entry.2 = std::cmp::max(entry.2, i);
                count
            })
            .values()
            .map(|(l, min, max)| (l, -1 * (max - min + 1) as i32))
            .max()
            .unwrap()
            .1
            * -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0697() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 3, 2, 2, 3, 1]), 2);
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
        assert_eq!(
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
            6
        );
    }
}
