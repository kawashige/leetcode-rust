use std::{cmp::Reverse, collections::BinaryHeap};

struct SeatManager {
    not_reserved: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        let mut not_reserved = BinaryHeap::new();
        for i in 1..=n {
            not_reserved.push(Reverse(i));
        }
        SeatManager { not_reserved }
    }

    fn reserve(&mut self) -> i32 {
        if let Some(Reverse(reserved)) = self.not_reserved.pop() {
            return reserved;
        }
        unreachable!()
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.not_reserved.push(Reverse(seat_number));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1845() {
        let mut obj = SeatManager::new(5);
        assert_eq!(obj.reserve(), 1);
        assert_eq!(obj.reserve(), 2);
        obj.unreserve(2);
        assert_eq!(obj.reserve(), 2);
        assert_eq!(obj.reserve(), 3);
        assert_eq!(obj.reserve(), 4);
        assert_eq!(obj.reserve(), 5);
        obj.unreserve(5);
    }
}
