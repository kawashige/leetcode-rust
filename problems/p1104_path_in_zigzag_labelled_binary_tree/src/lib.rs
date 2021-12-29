pub struct Solution {}

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut result = vec![label];
        while result.last() != Some(&1) {
            result.push(result.last().unwrap() / 2);
        }
        let depth = result.len();
        result
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, l)| {
                if i % 2 != depth % 2 {
                    l
                } else {
                    2_i32.pow(i as u32 + 1) + 2_i32.pow(i as u32) - l - 1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1104() {
        assert_eq!(Solution::path_in_zig_zag_tree(14), [1, 3, 4, 14]);
        assert_eq!(Solution::path_in_zig_zag_tree(26), [1, 2, 6, 10, 26]);
    }
}
