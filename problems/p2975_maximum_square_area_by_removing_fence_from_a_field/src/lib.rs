use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;

        if m == n {
            return (((m - 1) as usize * (n - 1) as usize) % M) as i32;
        }

        let mut h_fences = h_fences;
        h_fences.push(1);
        h_fences.push(m);
        h_fences.sort_unstable();
        let mut h_set = HashSet::new();
        for i in 0..h_fences.len() {
            for j in 0..i {
                h_set.insert(h_fences[i] - h_fences[j]);
            }
        }

        let mut v_fences = v_fences;
        v_fences.push(1);
        v_fences.push(n);
        v_fences.sort_unstable();
        let mut v_set = HashSet::new();
        for i in 0..v_fences.len() {
            for j in 0..i {
                v_set.insert(v_fences[i] - v_fences[j]);
            }
        }

        let mut h_interval = h_set.into_iter().collect::<Vec<_>>();
        h_interval.sort_unstable_by(|a, b| b.cmp(a));
        for interval in h_interval {
            if v_set.contains(&interval) {
                return ((interval as usize * interval as usize) % M) as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2975() {
        assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
        assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
    }
}
