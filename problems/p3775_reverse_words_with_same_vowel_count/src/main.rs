pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        let mut words = s
            .split_whitespace()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        let count = words[0].chars().filter(|c| vowels.contains(&c)).count();
        for i in 1..words.len() {
            if words[i].chars().filter(|c| vowels.contains(&c)).count() == count {
                words[i] = words[i].chars().rev().collect();
            }
        }

        words.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3775() {
        assert_eq!(
            Solution::reverse_words("cat and mice".to_string()),
            "cat dna mice".to_string()
        );
        assert_eq!(
            Solution::reverse_words("book is nice".to_string()),
            "book is ecin".to_string()
        );
        assert_eq!(
            Solution::reverse_words("banana healthy".to_string()),
            "banana healthy".to_string()
        );
    }
}

fn main() {}
