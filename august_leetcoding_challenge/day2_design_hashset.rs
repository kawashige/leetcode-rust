struct MyHashSet {
    data: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            data: Vec::new()
        }
    }
    
    fn add(&mut self, key: i32) {
        if self.data.len() == 0 {
            self.data.push(key);
        } else {
            match self.data.binary_search(&key) {
                Err(i) => self.data.insert(i, key),
                _ => {},
            }
        }
    }
    
    fn remove(&mut self, key: i32) {
        match self.data.binary_search(&key) {
            Ok(i) => {
                self.data.remove(i);
            },
            _ => {
                // nothing to do
            },
        }
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.data.contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
#[cfg(test)]
mod test {
    use super::MyHashSet;

    #[test]
    fn test_day2() {
        let mut hash_set = MyHashSet::new();
        hash_set.add(1);         
        hash_set.add(2);         
        assert_eq!(true, hash_set.contains(1));
        assert_eq!(false, hash_set.contains(3));
        hash_set.add(2);          
        assert_eq!(true, hash_set.contains(2));
        hash_set.remove(2);          
        assert_eq!(false, hash_set.contains(2));
    }
}