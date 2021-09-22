pub struct Solution {}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let bits = arr
            .into_iter()
            .filter_map(|w| {
                let bit = w
                    .chars()
                    .fold(0_usize, |bit, c| bit | 1 << (c as usize - 0x61));
                if bit.count_ones() == w.len() as u32 {
                    Some(bit)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut max = 0;

        for i in 0..2_usize.pow(bits.len() as u32) {
            let mut count = 0;
            let mut bit = 0;
            for j in 0..bits.len() {
                if i & 1 << j > 0 {
                    count += bits[j].count_ones();
                    bit |= bits[j];
                }
            }

            if bit.count_ones() == count {
                max = max.max(count);
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        assert_eq!(
            Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]),
            4
        );
        assert_eq!(
            Solution::max_length(vec![
                "cha".to_string(),
                "r".to_string(),
                "act".to_string(),
                "ers".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]),
            26
        );
    }
}
