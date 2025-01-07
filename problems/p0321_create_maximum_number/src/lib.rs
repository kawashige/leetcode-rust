use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        j: usize,
        remains: usize,
        len: usize,
        nums1: &[i32],
        nums2: &[i32],
        memo: &mut HashMap<(usize, usize, usize, usize), String>,
    ) -> String {
        if len == 0 {
            return Default::default();
        }
        if let Some(s) = memo.get(&(i, j, remains, len)) {
            return s.to_string();
        }
        let s = match (
            (i..=(i + remains).min(nums1.len() - 1)).max_by_key(|k| (nums1[*k], -1 * *k as i32)),
            (j..=(j + remains).min(nums2.len() - 1)).max_by_key(|k| (nums2[*k], -1 * *k as i32)),
        ) {
            (Some(k), Some(l)) => {
                let s1 = format!(
                    "{}{}",
                    nums1[k],
                    Self::recurse(k + 1, j, remains - (k - i), len - 1, nums1, nums2, memo)
                );
                let s2 = format!(
                    "{}{}",
                    nums2[l],
                    Self::recurse(i, l + 1, remains - (l - j), len - 1, nums1, nums2, memo)
                );
                s1.max(s2)
            }
            (Some(k), _) => {
                format!(
                    "{}{}",
                    nums1[k],
                    Self::recurse(k + 1, j, remains - (k - i), len - 1, nums1, nums2, memo)
                )
            }
            (_, Some(l)) => {
                format!(
                    "{}{}",
                    nums2[l],
                    Self::recurse(i, l + 1, remains - (l - j), len - 1, nums1, nums2, memo)
                )
            }
            _ => Default::default(),
        };
        memo.insert((i, j, remains, len), s.to_string());
        s
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let max_num = Self::recurse(
            0,
            0,
            nums1.len() + nums2.len() - k as usize,
            k as usize,
            &nums1,
            &nums2,
            &mut HashMap::new(),
        );

        max_num
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0321() {
        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
        assert_eq!(
            Solution::max_number(vec![3, 9], vec![8, 9], 3),
            vec![9, 8, 9]
        );
    }
}
