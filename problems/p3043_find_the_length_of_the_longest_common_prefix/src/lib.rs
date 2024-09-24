pub struct Solution {}

#[derive(Clone, Debug)]
struct Trie {
    children: Vec<Option<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: vec![None; 10],
        }
    }

    fn insert(&mut self, num: i32) {
        let mut trie = self;
        for d in num
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as usize)
        {
            trie = trie.children[d].get_or_insert(Trie::new());
        }
    }

    fn find(&self, num: i32) -> i32 {
        let mut trie = self;
        let mut len = 0;
        for d in num
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as usize)
        {
            if let Some(t) = trie.children[d].as_ref() {
                len += 1;
                trie = t;
            } else {
                break;
            }
        }
        len
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for num in arr1 {
            trie.insert(num);
        }
        arr2.into_iter().map(|num| trie.find(num)).max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3043() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
            0
        );
    }
}
