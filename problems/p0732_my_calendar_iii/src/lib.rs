use std::collections::BTreeMap;

struct MyCalendarThree {
    events: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.events.entry(start).or_insert(0) += 1;
        *self.events.entry(end).or_insert(0) -= 1;

        let mut max_booking = 0;
        let mut booking = 0;
        for (_, v) in &self.events {
            booking += v;
            max_booking = max_booking.max(booking);
        }
        max_booking
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0732() {
        let mut obj = MyCalendarThree::new();
        assert_eq!(obj.book(24, 40), 1);
        assert_eq!(obj.book(43, 50), 1);
        assert_eq!(obj.book(27, 43), 2);
        assert_eq!(obj.book(5, 21), 2);
        assert_eq!(obj.book(30, 40), 3);
        assert_eq!(obj.book(14, 29), 3);
        assert_eq!(obj.book(3, 19), 3);
        assert_eq!(obj.book(3, 14), 3);
        assert_eq!(obj.book(25, 39), 4);
        assert_eq!(obj.book(6, 19), 4);

        let mut obj = MyCalendarThree::new();
        assert_eq!(obj.book(10, 20), 1);
        assert_eq!(obj.book(50, 60), 1);
        assert_eq!(obj.book(10, 40), 2);
        assert_eq!(obj.book(5, 15), 3);
        assert_eq!(obj.book(5, 10), 3);
        assert_eq!(obj.book(25, 55), 3);
    }
}
