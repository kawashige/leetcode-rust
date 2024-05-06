use std::collections::VecDeque;

struct DataStream {
    value: i32,
    k: i32,
    values: VecDeque<(i32, i32)>,
    value_count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            value,
            k,
            values: VecDeque::new(),
            value_count: 0,
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if self.value_count == 0 || self.values[self.values.len() - 1].0 != num {
            self.values.push_back((num, 1));
        } else {
            let l = self.values.len() - 1;
            self.values[l].1 += 1;
        }
        self.value_count += 1;

        if self.k < self.value_count {
            if self.values[0].1 == 1 {
                self.values.pop_front();
            } else {
                self.values[0].1 -= 1;
            }
            self.value_count -= 1;
        }

        self.value_count == self.k && self.values.len() == 1 && self.values[0].0 == self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2526() {
        let mut obj = DataStream::new(4, 3);
        assert!(!obj.consec(4));
        assert!(!obj.consec(4));
        assert!(obj.consec(4));
        assert!(!obj.consec(3));
    }
}
