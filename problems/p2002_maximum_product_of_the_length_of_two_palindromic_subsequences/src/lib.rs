pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: &str, i: usize) -> bool {
        let indices = (0..s.len()).filter(|j| i & 1 << j != 0).collect::<Vec<_>>();

        for j in 0..indices.len() / 2 {
            if s.as_bytes()[indices[j]] != s.as_bytes()[indices[indices.len() - 1 - j]] {
                return false;
            }
        }

        true
    }

    pub fn max_product(s: String) -> i32 {
        let mut palindromes = Vec::new();

        for i in 1..2_usize.pow(s.len() as u32) {
            if Self::is_palindrome(&s, i) {
                palindromes.push(i);
            }
        }

        let mut max_product = 1;

        println!("palindromes: {:?}", palindromes);
        for p in &palindromes {
            println!("{:b}", p);
        }

        for i in 0..palindromes.len() {
            for j in i + 1..palindromes.len() {
                if palindromes[i] & palindromes[j] == 0 {
                    max_product = max_product.max(
                        palindromes[i].count_ones() as i32 * palindromes[j].count_ones() as i32,
                    );
                }
            }
        }

        max_product
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2002() {
        assert_eq!(Solution::max_product("leetcodecom".to_string()), 9);
        assert_eq!(Solution::max_product("bb".to_string()), 1);
        assert_eq!(Solution::max_product("accbcaxxcxx".to_string()), 25);
    }
}
