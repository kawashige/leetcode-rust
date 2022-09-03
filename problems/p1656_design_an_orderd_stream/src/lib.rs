struct OrderedStream {
    order: Vec<String>,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            order: vec![String::new(); n as usize + 1],
            ptr: 0,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.order[id_key as usize - 1] = value;
        if self.order[self.ptr].is_empty() {
            Default::default()
        } else {
            let left = self.ptr;
            let right = (left + 1..self.order.len())
                .find(|i| self.order[*i].is_empty())
                .unwrap();
            self.ptr = right;
            self.order[left..right].iter().cloned().collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1656() {
        let mut obj = OrderedStream::new(5);
        assert_eq!(obj.insert(3, "ccccc".to_string()), vec![] as Vec<String>);
        assert_eq!(
            obj.insert(1, "aaaaa".to_string()),
            vec!["aaaaa".to_string()] as Vec<String>
        );
        assert_eq!(
            obj.insert(2, "bbbbb".to_string()),
            vec!["bbbbb".to_string(), "ccccc".to_string()] as Vec<String>
        );
        assert_eq!(obj.insert(5, "eeeee".to_string()), vec![] as Vec<String>);
        assert_eq!(
            obj.insert(4, "ddddd".to_string()),
            vec!["ddddd".to_string(), "eeeee".to_string()] as Vec<String>
        );
    }
}
