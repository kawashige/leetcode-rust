use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        if m == arr.len() as i32 {
            return arr.len() as i32;
        }

        let mut len = vec![0; arr.len() + 1];
        let mut map = BTreeMap::new();
        map.insert(arr.len() - 1, 0);
        *len.last_mut().unwrap() += 1;

        for j in (0..arr.len()).rev() {
            let i = arr[j] as usize - 1;
            let (right, left) = map.range(i..).next().unwrap();
            let left = *left;
            let right = *right;

            len[right - left + 1] -= 1;
            if i == left && i == right {
                map.remove(&right);
            } else if i == left {
                map.insert(right, i + 1);
                len[right - (i + 1) + 1] += 1;
            } else if i == right {
                map.remove(&right);
                map.insert(i - 1, left);
                len[i - 1 - left + 1] += 1;
            } else {
                map.insert(i - 1, left);
                map.insert(right, i + 1);
                len[i - 1 - left + 1] += 1;
                len[right - (i + 1) + 1] += 1;
            }

            if 0 < len[m as usize] {
                return j as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1562() {
        assert_eq!(Solution::find_latest_step(vec![1, 2], 1), 1);
        assert_eq!(Solution::find_latest_step(vec![3, 5, 1, 2, 4], 1), 4);
        assert_eq!(Solution::find_latest_step(vec![3, 1, 5, 4, 2], 2), -1);
    }
}
