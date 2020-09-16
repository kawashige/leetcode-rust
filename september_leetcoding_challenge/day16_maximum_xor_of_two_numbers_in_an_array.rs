pub struct Solution {}

struct Trie {
    children: Vec<Option<Box<Trie>>>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: vec![None, None],
        }
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for n in &nums {
            let mut node = &mut trie;
            for i in (0..31).rev() {
                let b = (n >> i & 1) as usize;
                node = node.children[b].get_or_insert(Trie::new().into());
            }
        }

        let mut result = 0;
        for n in nums {
            let mut max = 0;
            let mut node = &trie;
            for i in (0..31).rev() {
                let b = (n >> i & 1) as usize;
                if let Some(n) = &node.children[1 - b] {
                    max |= 1 << i;
                    node = n;
                } else {
                    node = &node.children[b].as_ref().unwrap();
                }
            }
            result = std::cmp::max(max, result);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day16() {
        assert_eq!(28, Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
        assert_eq!(31, Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 6]));
        assert_eq!(10, Solution::find_maximum_xor(vec![8, 10, 2]));
    }
}
