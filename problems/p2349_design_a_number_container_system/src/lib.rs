use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Default)]
struct NumberContainers {
    indices: HashMap<i32, i32>,
    numbers: HashMap<i32, BinaryHeap<Reverse<i32>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Default::default()
    }

    fn change(&mut self, index: i32, number: i32) {
        self.indices.insert(index, number);
        (*self.numbers.entry(number).or_insert(BinaryHeap::new())).push(Reverse(index));
    }

    fn find(&mut self, number: i32) -> i32 {
        if let Some(indices) = self.numbers.get_mut(&number) {
            while let Some(Reverse(index)) = indices.pop() {
                if self.indices[&index] == number {
                    indices.push(Reverse(index));
                    return index;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2349() {
        let mut obj = NumberContainers::new();
        assert_eq!(obj.find(10), -1);
        obj.change(2, 10);
        obj.change(1, 10);
        obj.change(3, 10);
        obj.change(5, 10);
        assert_eq!(obj.find(10), 1);
        obj.change(1, 20);
        assert_eq!(obj.find(10), 2);
    }
}
