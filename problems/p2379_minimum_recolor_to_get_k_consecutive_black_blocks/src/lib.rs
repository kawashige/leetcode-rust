pub struct Solution {}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut count = vec![0; blocks.len() + 1];
        for i in 0..blocks.len() {
            count[i + 1] += count[i];
            count[i + 1] += if blocks.as_bytes()[i] == b'B' { 0 } else { 1 };
        }
        let mut min = std::i32::MAX;
        for i in k as usize..=blocks.len() {
            min = min.min(count[i] - count[i - k as usize]);
        }
        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2379() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
    }
}
