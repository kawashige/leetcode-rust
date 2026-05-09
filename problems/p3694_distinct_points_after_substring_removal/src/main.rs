use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn distinct_points(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();
        let mut coodinates = vec![(0, 0); n + 1];
        let mut set = HashSet::new();

        for i in 0..n {
            coodinates[i + 1] = coodinates[i];
            match s.as_bytes()[i] {
                b'U' => coodinates[i + 1].1 += 1,
                b'D' => coodinates[i + 1].1 -= 1,
                b'L' => coodinates[i + 1].0 -= 1,
                b'R' => coodinates[i + 1].0 += 1,
                _ => unreachable!(),
            };
        }
        for i in k - 1..n {
            let mut pos = coodinates[i + 1 - k];
            pos.0 += coodinates[n].0 - coodinates[i + 1].0;
            pos.1 += coodinates[n].1 - coodinates[i + 1].1;
            set.insert(pos);
        }

        return set.len() as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3694() {
        assert_eq!(Solution::distinct_points("LUL".to_string(), 1), 2);
        assert_eq!(Solution::distinct_points("UDLR".to_string(), 4), 1);
        assert_eq!(Solution::distinct_points("UU".to_string(), 1), 1);
    }
}

fn main() {}
