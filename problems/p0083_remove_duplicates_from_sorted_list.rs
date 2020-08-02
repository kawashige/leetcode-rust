#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn delete(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                Some(node) => {
                    let mut result = ListNode::new(node.val);
                    let mut next = node.next;

                    while let Some(next_node) = next {
                        if next_node.val != node.val {
                            result.next = delete(Some(next_node));
                            break;
                        }
                        next = next_node.next;
                    }

                    Some(Box::new(result))
                }
                None => None,
            }
        }

        delete(head)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_83() {
        let mut l1_1 = ListNode::new(1);
        let mut l1_2 = ListNode::new(1);
        let l1_3 = ListNode::new(4);
        l1_2.next = Some(Box::new(l1_3));
        l1_1.next = Some(Box::new(l1_2));

        let result = Solution::delete_duplicates(Some(Box::new(l1_1))).unwrap();

        let mut l2_1 = ListNode::new(1);
        let l2_2 = ListNode::new(4);
        l2_1.next = Some(Box::new(l2_2));

        assert_eq!(l2_1, *result);
    }
}
