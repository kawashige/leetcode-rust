use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct StockPrice {
    latest: (i32, i32),
    maximum: BinaryHeap<(i32, i32, i32)>,
    minimum: BinaryHeap<(Reverse<i32>, i32, i32)>,
    versions: HashMap<i32, i32>,
}

impl StockPrice {
    fn new() -> Self {
        Self {
            latest: (0, 0),
            maximum: BinaryHeap::new(),
            minimum: BinaryHeap::new(),
            versions: HashMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if self.latest.0 <= timestamp {
            self.latest = (timestamp, price);
        }
        let version = self.versions.get(&timestamp).unwrap_or(&0) + 1;
        self.maximum.push((price, timestamp, version));
        self.minimum.push((Reverse(price), timestamp, version));
        self.versions.insert(timestamp, version);
    }

    fn current(&self) -> i32 {
        self.latest.1
    }

    fn maximum(&mut self) -> i32 {
        while let Some((p, t, v)) = self.maximum.pop() {
            if self.versions[&t] == v {
                self.maximum.push((p, t, v));
                return p;
            }
        }
        unreachable!()
    }

    fn minimum(&mut self) -> i32 {
        while let Some((Reverse(p), t, v)) = self.minimum.pop() {
            if self.versions[&t] == v {
                self.minimum.push((Reverse(p), t, v));
                return p;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2034() {
        let mut obj = StockPrice::new();
        obj.update(1, 10);
        obj.update(2, 5);
        assert_eq!(obj.current(), 5);
        assert_eq!(obj.maximum(), 10);
        obj.update(1, 3);
        assert_eq!(obj.maximum(), 5);
        obj.update(4, 2);
        assert_eq!(obj.minimum(), 2);
    }
}
