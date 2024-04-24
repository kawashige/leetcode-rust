pub struct Solution {}

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut prev = 1;
        let mut count = std::i32::MIN;
        let mut result = 0;

        for i in 0..forts.len() {
            if forts[i] == 0 {
                count += 1;
            } else {
                if forts[i] != prev {
                    result = count.max(result);
                }
                count = 0;
                prev = forts[i];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2511() {
        assert_eq!(Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
        assert_eq!(Solution::capture_forts(vec![0, 0, 1, -1]), 0);
    }
}
