use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}
#[derive(Debug)]
struct MyLinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    fn get(&self, index: i32) -> i32 {
        if index < 0 || self.len as i32 - 1 < index {
            return -1;
        }

        let mut node = self.head.clone();
        for _ in 0..index {
            let next = node.unwrap().borrow().next.clone();
            node = next;
        }
        node.clone().unwrap().borrow().val
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        let head = self.head.take();
        let mut new_node = ListNode::new(val);
        new_node.next = head;
        self.head = Some(Rc::new(RefCell::new(new_node)));
        self.len += 1;
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }

    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        if self.len == 0 {
            self.add_at_head(val);
        } else {
            let new_node = Some(Rc::new(RefCell::new(ListNode::new(val))));
            self.tail.as_ref().unwrap().borrow_mut().next = new_node.clone();
            self.tail = new_node;
            self.len += 1;
        }
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    fn add_at_index(&mut self, index: i32, val: i32) {
        if (self.len as i32) < index {
            return;
        }

        if index == 0 {
            self.add_at_head(val);
        } else if index == self.len as i32 {
            self.add_at_tail(val);
        } else {
            let mut node = self.head.clone();
            for _ in 0..(index - 1) {
                let next = node.unwrap().borrow().next.clone();
                node = next;
            }

            let mut new_node = ListNode::new(val);
            new_node.next = node.clone().unwrap().borrow_mut().next.take();
            node.clone().unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
            self.len += 1;
        }
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    fn delete_at_index(&mut self, index: i32) {
        if self.len as i32 - 1 < index {
            return;
        }

        self.len -= 1;

        if index == 0 {
            let head = self.head.clone().take();
            self.head = head.unwrap().borrow_mut().next.take();
        } else {
            let mut node = self.head.clone();
            for _ in 0..(index - 1) {
                let next = node.unwrap().borrow().next.clone();
                node = next;
            }
            let delete_node = node.clone().unwrap().borrow_mut().next.take();
            node.clone().unwrap().borrow_mut().next = delete_node.unwrap().borrow_mut().next.take();
            if node.clone().unwrap().borrow().next.is_none() {
                self.tail = node.clone();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0707() {
        // let mut obj = MyLinkedList::new();
        // obj.add_at_head(7);
        // obj.add_at_head(2);
        // obj.add_at_head(1);
        // obj.add_at_index(3, 0);
        // obj.delete_at_index(2);
        // obj.add_at_head(6);
        // obj.add_at_tail(4);
        // assert_eq!(obj.get(4), 4);
        // obj.add_at_head(4);
        // obj.add_at_index(5, 0);
        // obj.add_at_head(6);

        let mut obj = MyLinkedList::new();
        obj.add_at_head(84);
        obj.add_at_tail(2);
        obj.add_at_tail(39);
        assert_eq!(obj.get(3), -1);
        assert_eq!(obj.get(1), 2);
        obj.add_at_tail(42);
    }
}
