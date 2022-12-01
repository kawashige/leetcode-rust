use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct AuthenticationManager {
    time_to_live: i32,
    tokens: HashMap<String, i32>,
    heap: BinaryHeap<(Reverse<i32>, String)>,
}

impl AuthenticationManager {
    fn new(timeToLive: i32) -> Self {
        Self {
            time_to_live: timeToLive,
            tokens: HashMap::new(),
            heap: BinaryHeap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens
            .insert(token_id.clone(), current_time + self.time_to_live);
        self.heap
            .push((Reverse(current_time + self.time_to_live), token_id));
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        self.expire(current_time);
        if self.tokens.contains_key(&token_id) {
            self.tokens
                .insert(token_id.clone(), current_time + self.time_to_live);
            self.heap
                .push((Reverse(current_time + self.time_to_live), token_id));
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        self.expire(current_time);
        self.tokens.len() as i32
    }

    fn expire(&mut self, current_time: i32) {
        while let Some((Reverse(t), id)) = self.heap.pop() {
            if current_time < t {
                self.heap.push((Reverse(t), id));
                break;
            }
            if let Some(limit) = self.tokens.get(&id) {
                if limit == &t {
                    self.tokens.remove(&id);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1797() {
        let mut obj = AuthenticationManager::new(5);
        obj.renew("aaa".to_string(), 1);
        obj.generate("aaa".to_string(), 2);
        assert_eq!(obj.count_unexpired_tokens(6), 1);
        obj.generate("bbb".to_string(), 7);
        obj.renew("aaa".to_string(), 8);
        obj.renew("bbb".to_string(), 10);
        assert_eq!(obj.count_unexpired_tokens(15), 0);
    }
}
