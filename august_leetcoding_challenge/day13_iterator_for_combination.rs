struct CombinationIterator {
    combination: Vec<String>,
    next: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        fn generate_combination(chars: Vec<char>, head: String, length: i32) -> Vec<String> {
            if length == 0 {
                return vec![head];
            }

            let mut result = Vec::new();
            result.append(&mut generate_combination(
                chars[1..].to_vec(),
                format!("{}{}", head, chars[0]),
                length - 1,
            ));
            if chars.len() > length as usize {
                result.append(&mut generate_combination(chars[1..].to_vec(), head, length));
            }
            result
        }
        CombinationIterator {
            combination: generate_combination(
                characters.chars().collect::<Vec<char>>(),
                "".to_string(),
                combinationLength,
            ),
            next: 0,
        }
    }

    fn next(&mut self) -> String {
        let tmp = self.next;
        self.next += 1;
        self.combination[tmp].clone()
    }

    fn has_next(&self) -> bool {
        self.next < self.combination.len()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod test {
    use super::CombinationIterator;

    #[test]
    fn test_day13() {
        let mut obj = CombinationIterator::new("abc".to_string(), 2);
        assert_eq!("ab".to_string(), obj.next());
        assert!(obj.has_next());
        assert_eq!("ac".to_string(), obj.next());
        assert!(obj.has_next());
        assert_eq!("bc".to_string(), obj.next());
        assert!(!obj.has_next());

        let mut obj = CombinationIterator::new("abc".to_string(), 3);
        assert_eq!("abc".to_string(), obj.next());
        assert!(!obj.has_next());
    }
}
