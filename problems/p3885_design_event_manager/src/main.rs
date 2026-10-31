use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct EventManager {
    priority: HashMap<i32, i32>,
    heap: BinaryHeap<(i32, Reverse<i32>)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl EventManager {
    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut priority = HashMap::new();
        let mut heap = BinaryHeap::new();

        for e in events {
            priority.insert(e[0], e[1]);
            heap.push((e[1], Reverse(e[0])));
        }

        Self { priority, heap }
    }

    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        self.priority.insert(event_id, new_priority);
        self.heap.push((new_priority, Reverse(event_id)));
    }

    fn poll_highest(&mut self) -> i32 {
        while let Some((p, Reverse(i))) = self.heap.pop() {
            if self.priority.get(&i) == Some(&p) {
                self.priority.remove(&i);
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3885() {
        let mut obj = EventManager::new(vec![vec![5, 7], vec![2, 7], vec![9, 4]]);
        assert_eq!(obj.poll_highest(), 2);
        obj.update_priority(9, 7);
        assert_eq!(obj.poll_highest(), 5);
        assert_eq!(obj.poll_highest(), 9);

        let mut obj = EventManager::new(vec![vec![4, 1], vec![7, 2]]);
        assert_eq!(obj.poll_highest(), 7);
        assert_eq!(obj.poll_highest(), 4);
        assert_eq!(obj.poll_highest(), -1);
    }
}

fn main() {}
