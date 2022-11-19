pub struct Solution {}

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut counts = vec![[0; 26]; s.len() + 1];
        let mut count = [0; 26];
        for i in 0..s.len() {
            count[(s.as_bytes()[i] - b'a') as usize] += 1;
            counts[i + 1] = count;
        }

        println!("counts: {:?}", counts);

        let mut beatuy = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                let mut min = std::i32::MAX;
                let mut max = 0;
                for k in 0..26 {
                    let c = counts[j + 1][k] - counts[i][k];
                    if 0 < c {
                        min = min.min(c);
                        max = max.max(c);
                    }
                }
                beatuy += max - min;
            }
        }
        beatuy
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1781() {
        assert_eq!(Solution::beauty_sum("aabcb".to_string()), 5);
        assert_eq!(Solution::beauty_sum("aabcbaa".to_string()), 17);
    }
}
