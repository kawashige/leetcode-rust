use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        (*self.map.entry(key).or_insert(Vec::new())).push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key).map_or(String::new(), |v| {
            match v.binary_search_by_key(&timestamp, |v| v.0) {
                Ok(i) => v[i].1.clone(),
                Err(i) => {
                    if i == 0 {
                        String::new()
                    } else {
                        v[i - 1].1.clone()
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0981() {
        let mut obj = TimeMap::new();
        obj.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(obj.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(obj.get("foo".to_string(), 3), "bar".to_string());
        obj.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(obj.get("foo".to_string(), 3), "bar".to_string());
        assert_eq!(obj.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(obj.get("foo".to_string(), 5), "bar2".to_string());
    }
}
