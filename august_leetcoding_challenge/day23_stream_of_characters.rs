use std::{collections::HashMap, vec::Vec};
struct StreamChecker {
    queries: Vec<char>,
    trie: HashMap<char, Trie>,
}

#[derive(Clone, Debug)]
struct Trie {
    is_end: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            is_end: false,
            children: HashMap::new(),
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut root = HashMap::new();
        for word in words {
            let chars = word.chars().rev().collect::<Vec<char>>();
            let mut trie = root.entry(chars[0]).or_insert(Trie::new());
            for i in 1..chars.len() {
                trie = trie.children.entry(chars[i]).or_insert(Trie::new());
            }
            trie.is_end = true;
        }

        Self {
            trie: root,
            queries: Vec::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.queries.push(letter);
        match self.trie.get(&letter) {
            Some(t) => {
                if t.is_end {
                    return true;
                }
                let mut trie = t;
                for i in (0..(self.queries.len() - 1)).rev() {
                    match trie.children.get(&self.queries[i]) {
                        Some(tt) => {
                            if tt.is_end {
                                return true;
                            }
                            trie = tt;
                        }
                        None => break,
                    }
                }
            }
            None => {}
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day23() {
        let mut obj = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
        assert!(!obj.query('a'));
        assert!(!obj.query('b'));
        assert!(!obj.query('c'));
        assert!(obj.query('d'));
        assert!(!obj.query('e'));
        assert!(obj.query('f'));
        assert!(!obj.query('a'));
        assert!(!obj.query('g'));
        assert!(!obj.query('h'));
        assert!(!obj.query('i'));
        assert!(!obj.query('j'));
        assert!(!obj.query('k'));
        assert!(obj.query('l'));

        let mut obj2 = StreamChecker::new(vec![
            "ab".to_string(),
            "ba".to_string(),
            "aaab".to_string(),
            "abab".to_string(),
            "baa".to_string(),
        ]);
        assert!(!obj2.query('a'));
        assert!(!obj2.query('a'));
        assert!(!obj2.query('a'));
        assert!(!obj2.query('a'));
        assert!(!obj2.query('a'));
        assert!(obj2.query('b'));

        let mut obj3 = StreamChecker::new(vec![
            "bbbbababbbabbaaaababbaabbbabbaaaababbabbaaaaaaaababababaab".to_string(),
            "babbbaaaababaaabbbbabbbaaabaabaaaabaababbbaaaaabbbabaabbbaabbbaabbaabbaabbbabababaabaaaaabbbaaaaaa".to_string(),
            "babababbaaaaaaaabbbbababaabaabbaabaaaaababbbaaabbbababaabbbaaaaabbbaabaaababaaba".to_string(),
            "bbbbaaabababbabababbbabbaabbaaabbbaabaaaaaaabbbbaaaabbabaaabaababbbbabaabaabb".to_string(),
            "bbbbaababbbaaabbaababbaabbbbbababaaabaaabbbbabbaaab".to_string(),
            "baabbaabbaaababaaaaabbbbbabaabaabbbbbaaabbabbbababaaaba".to_string(),
            "baabbabbabaababababbabbbbbaaaabaabbbbbababbbbbabaaaabbaaababbaab".to_string(),
            "babbbaaaaabbbbbaababaabababbbbbbabaabbaabaababbbbbbaaabaaaaaaabbbababaabbbaaaaaababbbbaabbbbaabba".to_string(),
            "abbaababbbabbaabababbbbbaabaaaaaababbaabbaaabbabbbbaabababbabaabbabbaabaababbaabaabbbbbabbbababbbb".to_string(),
            "aaabbbaaababbbaabbbbaaaaababaababaabbaaaabaaabbaaaaaaa".to_string(),
            "bbbabbaaaabbababaabaaaaaabbbaabbbaabbbabbbabaabbaabaababbaaababaababbaaabb".to_string(),
            "abaaaabaaabbbaaababbbbbaaaabbabbbbbbabbbbbbabaabbbaabaa".to_string(),
            "baabbabababaaaaabbbbababbbaabbbaaabaabbbbabbaaababbaabaaaababaabaaaab".to_string(),
            "ababbaaababaabbbabaabbaabbbaababbbabaabbbaaabaababbaabbbbbabaaaabbbbbaaabab".to_string(),
            "aaaabaaabbbabaabaaaabbbbabaaaaabababbbbbbaaabbbbaabbababbbbbabbbbabbbbaababababbabbaaaaa".to_string(),
            "baabbbbaaabbaaabbaabababaabbabaaababbbabbbaabbabbaabaaaabbabaabaababb".to_string(),
            "baaabbabbbbabaabbaababbabaabbaababbbabbabbbbababbbbbabbabbaabbaabbbbababbabbbbabba".to_string(),
            "aaabbbabbbaabbabbabbaaaabbaaaaaababbbababaabaabbababbbbaaaababbbbaaaaaa".to_string(),
            "abbabaaabbbbbaaabababbababaabbbaaabbbbabbbababbababbbbbbbabbabaabbbbaaaaabbabbaaaabababaaaaabab".to_string(),
            "bbbaaaaabbbabababbbaaababababbbbbaaaaabbbbbbabaaababaababbabbabbaaaaaabababaaabba".to_string(),
            "bbabbbaaaaabbaaabababbaabbbbbbbbbbbaaabbbababbbbabaa".to_string(),
            "babaabbbaaabbbabbbbbaabaabbbabaaabbbaababbbbbabbbbaaabaababbaaaaabbbbbaabbabbbbaababaababaabbbaa".to_string(),
            "abababbbbabababaabbabbaabbbaabaaabbbaababbabbbbbaabbabbaabababbaabbaabaaabaabaabbaaaabbbab".to_string(),
            "abaabababbbbababaaaabaabaabaaabbbabbbbaababaaaababbaabbabaabbbbaaaaabbaabaabbaabbb".to_string(),
            "aabbaaaabbaababaaaaaaababbabaabbbaabaaabbaaabbbbabbabbaaabbaaaabbabababbabbabaababb".to_string(),
            "bbbbaabaabbbabaabbababaaababbabbbbbbaaabaaaaaabaabaaaabbaaabb".to_string(),
            "abaabbbbbaaabbbababbababbbbbbbaaaabbbabbaabbaaabbabaabbaabbbbaabbab".to_string(),
            "abaaababbaaaabbbababbabaaaaabaabbbbabababbbaaaaabaabbaaabbbabbabbbaaaa".to_string(),
            "bbabbabbabaaabbabaaaabaaabaaaaaaabbbbbabbbbaaababbbaaaaabbb".to_string(),
            "aabaababbaabbaabbbbaabbbbabbbabbbabbabbbbbbabbbbbababababbabbabaaabaaabbabbaba".to_string(),
            "abbbbaaaabbbaabababbababbaabaaaabbbaaaaabbababaaaabbaaaaa".to_string(),
            "abbababbababababbababbaabbbaaaababbaaabaababbaaaabbaababababaa".to_string(),
            "abbaaaabbbaabbaaaabababbbbbaabbabbbbabbabbbaaabbabbbbbabaabbbabaaaaabaabaaaaaaaab".to_string(),
            "abaaababbabbaababababbbaabaababbaaabbbbaaaabbababaabbbbbbaabaaababaabbbabaabbbbbbbbababbbbb".to_string(),
            "babbbabbaaabbabbabbbaababbaabaaaaaaabbbaababbbabbaabbbbbbbabbaaaaa".to_string(),
            "bbbaababababababbabbbaabbbbbaabbbaaabababaaaabbabbaabbbbaaababbabbababaaaaaaaabbbbaaaababbba".to_string(),
            "bababbabaaabaaaaaabbbabaaaaabbbbbbbabaaaaabbbbabbbbbb".to_string(),
            "aabbaaabbabbaabababbbbaaaabababbbabaababababababbaaabaaababbabbbaabbabbaaabbbba".to_string(),
            "aababaabaababaabbaabbaaaabaaabbaababbaaababbbaaaabaa".to_string(),
            "abbaabbbbaabaabbbababbaabbbbabbbaabbabaaaaaabbbaababababbababbabbabaaabbbbbba".to_string(),
            "bbabbabbaaabbbaaabbbbaaaabbaabababaaabaaaabaaaabaaabaabbbaabbaabbabbbbbbbabaaaaabbbaaabbaaabaaabb".to_string(),
            "aababbbaabbaabaabaaaaaabbbbbababbbabaababaaaaaabaaabababbbbaaababbabaaaaabbbbaaa".to_string(),
            "aabbabaaabbaaaababaaabbabaabaabaabaabbaabbbbabaaabbbbbbaaaabababbbbabba".to_string(),
            "babbbbbaabbbaaabbbabbabbbabaabbbaaabababbabbbaabababaaaaaabaaabbbaaaaaaaabababa".to_string(),
            "bbabbababababababaaaabbbbaabaabbbbbaaaabbaaaabababbbba".to_string(),
            "abbbbbabbbaaabbaaabbbaaababbbbbaabbbbabaababbabaabaaaababaa".to_string(),
            "aaabbbaabbbabbabbbabaabbaababbbababbaabbbababbbaabbaaabbbabaabaaabababbabbbbbabaa".to_string(),
            "bababbbbaabababbbaabaabaaabbababababaabaaababbbbaaabbbbbaba".to_string(),
            "aababababbaabbbbabbaababaaaabbbbbaabaabaabbbbaaaaabbababbbbbbaaaabaaaabbaabbbb".to_string(),
            "baabbbaaababbbbbbbbaababababbbbaaaaaaaababbaabaabbbaaabbabababb".to_string(),
            "abaaaababbbbbbabbbbaabbabbbabbbbbbbbbabaabbbababaabaaabaaabaaaaabaaabbaabbbaabaabbbbbbbaaabaa".to_string(),
            "aaaabababbababbabbabbaabbabbabbbaabbbabbbababaaabbbbbbbababaaaabaaaabaabaabaabbababbaabaabaa".to_string(),
            "abbaaabbbbbbbaaaabbabbaabababbabbaabbaaaababbabaaaaaaabbbabbb".to_string(),
            "bababbbbbaaabbbbaaabababaaaabaabbbabbbabbabbabaababaabbaaabaababbbabbaabbaabbabaabbab".to_string(),
            "aabaaabaababbbbababbbaabbbabbbabbbbbaababaabaaabaabababbbbabbbbbbbabbbbaabbaabbbaabbbbabbbaaaaa".to_string(),
            "baabbabbbaabababbaaaaabbabaababbbbbbaaaabbbbaaabaaaaabababbaabbabbabaaabbaabaaab".to_string(),
            "ababbaaabbbababbaabbbabbbaabbaababbaabaaabbaaaaabaaaaababaababaaabbabaababbaaababbbbaaababaaaaab".to_string(),
            "aabbaabaaaaabaabaaabaababaaababbbbaababaaababbabaababaaa".to_string(),
            "bbbaaaaaabababbbbaabbbabaaaabaaabaaabbbababbbaaababaababbaabb".to_string(),
            "abababaaabbbabbabaabbbababbaabbbaaabbbaabababaaaaabbbabaabbaaaaabaabaaab".to_string(),
            "aaaaaabbabbbabaabbabababaabbbbbabaabaabbbbbabbaaabbbaaabbbabbaabbabbaaaa".to_string(),
            "bbabbbaababaabbbbbbabaababbbaabbabaabbbaaaababaaaabaaabaaabbaaabaaaabbaaaababa".to_string(),
            "bbababaaabaaabaaaabaabaaabbbaabaaabaabaababbabbbabbbabaaabbabbaabbabaab".to_string(),
            "abbabaababbaababbbbabaabbbbabbbbabbbabbbababbbbaabaababbba".to_string(),
            "abbabbaaabaabbabbaaaabaababbbaaabbaaaabbababbbaabbaba".to_string(),
            "aaaaaaaababbbabbbaaaaaaaaabbaaaabbababaaaabbbaabbabbab".to_string(),
            "aabbabbbaaaabaaabbbaaabaaaaabbbbbaaaaababbabbbbabbababa".to_string(),
            "aabaaaaaaaaababababbabbaaaaabbabbaaaaaaababaabbbbbbbbbaaaababbbaabaabbaaaabababbbbbaaa".to_string(),
            "aabbaabbababbbbbabaaabbaaabbababbbbaabaabaabaaaabbabaaaaabbaabbbabbbabbbbaaaabbaaaabbaba".to_string(),
            "abbababbbbbaababbbaabbaaaabbaaababaabababbbabaaaaabbabaaab".to_string(),
            "babbbabaababbbbaaabababaabbaabbababbbabbbabbabaaababbaababbbba".to_string(),
            "bbbbaaaaababaabaabababaaaaaaaaaabbabaaabaababaaabababbbbbbbaaababbbbbaabbabababaabbabaabab".to_string(),
            "abbbbbbababbbabbaaaabbbbaabbbbbbbabbbabbbbbabaabbbbaaaaabbaababaabbbbbaaaabbaabaaababbbbaaaaabb".to_string(),
            "bbaaabbbababbaabbbbababaabababbabbbbbbbbababaabbbaaabbaaabaabbbababbaabbababbbaababaabbbabaaaaabbaa".to_string(),
            "abbabaabbaaabaaaabbaaabbabababaabbbabbaabaaabbbaaaba".to_string(),
            "abbbabaabbaabbaaabbabbabbaaabbabbaabbbababaaaabbaabbbbbbbaaababaaababbbab".to_string(),
            "aababbbaababbbbbbabbbabbaabbbabbaabbbbababaabaabaaaaba".to_string(),
            "babbaaaabaaabbaababbababaababaabbbaabbaababaabababbbabbababbbbaaababaab".to_string(),
            "abaaaaabaaabbbbabbaabbaabababbbabbabababbbabbabbbbaababbbabbaabbbaaaaabababbbbab".to_string(),
            "aaaaaaabaaaabbbbbabaaabababaaabaaaaababaaaaaababaababbabbbabbbabbbbbbaaaabaaabbbbaaaaabaababaabbbbb".to_string(),
            "aabaababbbbbbbabbaabaaabbbababaabbbbaaaaaaabbbbbbbbbabbbabaaaba".to_string(),
            "baaaaaaaaabaaaaababbbaaaabababbaabbaabbabababbabbabbbbbbabbaabbbbbabaaabbbbabbabbbabbbbbabaaa".to_string(),
            "bbbbbabbbbbaabbbaabbaaaaabbbbbabbbbbbababbabbbbbaaababaaaababaababbabbaaababbbabaaba".to_string(),
            "babaaaababaaababbbaaabbaabababbbbabbbabbaabbaaabababbbababbbbaabbababbbaabbbbbabbabaaaabbbababb".to_string(),
            "babaaabbbbbbaabbaaaababbabbaaaaabbbaabbabbbbabbbaabaaabaabbbbaba".to_string(),
            "aaabbabbbbaaaaaaaaabaabbbbbaabbaabaaababbbbbabaaaabbbabaabbbabbabbabbbbabababa".to_string(),
            "aababbabbbbababbbbbaabbabbabbababbaabbbaaababaabbbaabbbbbb".to_string(),
            "ababbabbaaaabaaabbbabaababbbaaaabbabbbbaaaaabbabaabbaabb".to_string(),
            "bababbbabbbaaaaaaaaaabaaaaaaababbabaaabaaabbbbbbbbabaaabaaaaabbbaabaabbbba".to_string(),
            "bbaababbbbababbbaaaabaabaabababbbabaabbaabaaabbbbabbaabaaababaabbbabbaaabbabaaababbbaaa".to_string(),
            "baaaabbabbabaaabaaaababababbbbaabbbbbbbaaababbbaaaaabbba".to_string(),
            "abbbaaaaabaaabbbbaaaaababbbbbbabbaaabbaababababaabbaaabaaaabaaaaaaabaaaabaabbbbababaaaababbba".to_string(),
            "abababaabababbbaaaaabbbbbabbbbaaababbbaaaababaabbabbaabababbaabbabaababbaaaaabbbb".to_string(),
            "bbbbbaabbbbabbbbabaaaabaabababaaaaabbabbabbaaaabaaaababbbbbbbbbbbaabbaaaaaababbbbb".to_string(),
            "ababbabbbbbabaababbbbabbaabbbababbbabbbbbababbbbbabaababbbbabaababbaabbbaaabbabbaabbaaabbbba".to_string(),
            "abaaabbbbbbabbaaabbbaababbbaababbabbbbbbabaabaabbbbbbaaabbaabababaababbaaabbaaabbbba".to_string(),
            "bbbaabaaaabbbbaababbababbaababbabaaaabaaabaabaaababbbbaababbabbbaaabbb".to_string(),
            "abaaaaaababaaaaabbbabaababbaaababaabbaabbbbabaaabbaabbaaaaabaaaaaaabaaabbaabaabbbbbaaa".to_string(),
            "aabaabbbaabaabbabaabababaababbbbaababbbababbaabaabaabbbbbbbbaabababaabbabaabaabbaababba".to_string(),
            "aaabaaaabbbaaaaaabaaabbbaabaababbaabbaabbbbbababaabbbbbababbbbbaababababaabaabaaabbbbbbababaabaaa".to_string()
        ]);
        assert!(!obj3.query('b'));
        assert!(!obj3.query('a'));
        assert!(!obj3.query('b'));
        assert!(!obj3.query('a'));
        assert!(!obj3.query('a'));
        assert!(!obj3.query('a'));
    }
}
