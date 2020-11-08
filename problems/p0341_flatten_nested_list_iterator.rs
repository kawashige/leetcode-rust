#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
use std::collections::VecDeque;
struct NestedIterator {
    list: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        fn flatten(nested_list: NestedInteger) -> VecDeque<i32> {
            let mut results = VecDeque::new();
            match nested_list {
                NestedInteger::List(list) => {
                    for l in list {
                        results.append(&mut flatten(l));
                    }
                }
                NestedInteger::Int(v) => results.push_back(v),
            }
            results
        }

        let mut list = VecDeque::new();
        for nl in nestedList {
            list.append(&mut flatten(nl));
        }

        Self { list }
    }

    fn next(&mut self) -> i32 {
        self.list.pop_front().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.list.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0341() {
        let mut obj = NestedIterator::new(vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ]);
        assert_eq!(1, obj.next());
        assert_eq!(1, obj.next());
        assert_eq!(2, obj.next());
        assert_eq!(1, obj.next());
        assert!(obj.has_next());
        assert_eq!(1, obj.next());
        assert!(!obj.has_next());
    }
}
