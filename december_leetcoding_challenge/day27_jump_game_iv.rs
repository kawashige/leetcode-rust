use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..arr.len() {
            (*map.entry(arr[i]).or_insert(HashSet::new())).insert(i);
        }
        let mut opened = HashSet::new();
        let mut next = vec![0].into_iter().collect::<HashSet<usize>>();
        let mut result = 0;
        while !next.contains(&(arr.len() - 1)) {
            if (1 < arr.len() && next.contains(&(arr.len() - 2)))
                || map[&arr[arr.len() - 1]].intersection(&next).count() > 0
            {
                result += 1;
                break;
            }

            let mut tmp = HashSet::new();
            for i in next {
                if opened.contains(&i) {
                    continue;
                }
                opened.insert(i);
                if 0 < i && !opened.contains(&(i - 1)) {
                    tmp.insert(i - 1);
                }
                if i < arr.len() - 1 && !opened.contains(&(i + 1)) {
                    tmp.insert(i + 1);
                }
                for j in &map[&arr[i]] {
                    if &i != j && !opened.contains(j) {
                        tmp.insert(*j);
                    }
                }
            }
            next = tmp;
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day27() {
        assert_eq!(
            3,
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
        );
        assert_eq!(0, Solution::min_jumps(vec![7]));
        assert_eq!(1, Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]));
        assert_eq!(2, Solution::min_jumps(vec![6, 1, 9]));
        assert_eq!(
            3,
            Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13])
        );
    }
}
