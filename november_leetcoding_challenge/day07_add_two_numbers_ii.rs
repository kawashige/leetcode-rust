// Definition for singly-linked list.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        } else if l2.is_none() {
            return l1;
        }

        let mut n1 = Vec::new();
        let mut node = l1;
        while let Some(n) = node {
            n1.push(n.val);
            node = n.next;
        }
        let mut n2 = Vec::new();
        let mut node = l2;
        while let Some(n) = node {
            n2.push(n.val);
            node = n.next;
        }

        let mut num = Vec::new();
        let mut overflow = 0;
        for i in 0..std::cmp::max(n1.len(), n2.len()) {
            let mut tmp = overflow;
            if i < n1.len() {
                tmp += n1[n1.len() - 1 - i]
            }
            if i < n2.len() {
                tmp += n2[n2.len() - 1 - i]
            }
            num.push(tmp % 10);
            overflow = tmp / 10;
        }
        if overflow != 0 {
            num.push(overflow);
        }
        num.reverse();

        let mut dummy = Box::new(ListNode::new(-1));
        let mut next = &mut dummy;
        for n in num {
            next = next.next.get_or_insert(Box::new(ListNode::new(n)));
        }

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day7() {
        let n1 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        }));
        let n2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let r = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(7))),
                })),
            })),
        }));

        assert_eq!(r, Solution::add_two_numbers(n1, n2));

        let n1 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode::new(9))),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let n2 = Some(Box::new(ListNode::new(7)));
        let r = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode {
                                            val: 0,
                                            next: Some(Box::new(ListNode::new(6))),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(r, Solution::add_two_numbers(n1, n2));
    }
}
