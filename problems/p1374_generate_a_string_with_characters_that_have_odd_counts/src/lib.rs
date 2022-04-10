pub struct Solution {}

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n & 1 == 0 {
            std::iter::repeat('a')
                .take(n as usize - 1)
                .chain(std::iter::once('b'))
                .collect()
        } else {
            std::iter::repeat('a').take(n as usize).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1374() {
        assert_eq!(Solution::generate_the_string(4), "aaab".to_string());
        assert_eq!(Solution::generate_the_string(2), "ab".to_string());
        assert_eq!(Solution::generate_the_string(7), "aaaaaaa".to_string());
    }
}
