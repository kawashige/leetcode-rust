use std::collections::{HashMap, HashSet, VecDeque};

struct Router {
    packets: VecDeque<(i32, i32, i32)>,
    packets_by_dest: HashMap<i32, VecDeque<(i32, i32)>>,
    set: HashSet<(i32, i32, i32)>,
    limit: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memoryLimit: i32) -> Self {
        Self {
            packets: VecDeque::new(),
            packets_by_dest: HashMap::new(),
            set: HashSet::new(),
            limit: memoryLimit as usize,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if self.set.insert((source, destination, timestamp)) {
            self.packets.push_back((source, destination, timestamp));
            (*self
                .packets_by_dest
                .entry(destination)
                .or_insert(VecDeque::new()))
            .push_back((source, timestamp));
            if self.packets.len() > self.limit {
                let p = self.packets.pop_front().unwrap();
                self.packets_by_dest.get_mut(&p.1).unwrap().pop_front();
                self.set.remove(&p);
            }
            true
        } else {
            false
        }
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((s, d, t)) = self.packets.pop_front() {
            self.packets_by_dest.get_mut(&d).unwrap().pop_front();
            self.set.remove(&(s, d, t));
            vec![s, d, t]
        } else {
            Default::default()
        }
    }

    fn get_count(&mut self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if !self.packets_by_dest.contains_key(&destination) {
            return 0;
        }
        if self.packets_by_dest[&destination].is_empty() {
            return 0;
        }

        (self.binary_search(destination, end_time + 1)
            - self.binary_search(destination, start_time)) as i32
    }

    fn binary_search(&self, destination: i32, val: i32) -> usize {
        if val <= self.packets_by_dest[&destination][0].1 {
            return 0;
        }
        if self.packets_by_dest[&destination].back().unwrap().1 < val {
            return self.packets_by_dest[&destination].len();
        }
        let mut ng = 0;
        let mut ok = self.packets_by_dest[&destination].len() - 1;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if val <= self.packets_by_dest[&destination][mid].1 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3508() {
        // let mut obj = Router::new(3);
        // assert!(obj.add_packet(1, 4, 90));
        // assert!(obj.add_packet(2, 5, 90));
        // assert!(!obj.add_packet(1, 4, 90));
        // assert!(obj.add_packet(3, 5, 95));
        // assert!(obj.add_packet(4, 5, 105));
        // assert_eq!(obj.forward_packet(), vec![2, 5, 90]);
        // assert!(obj.add_packet(5, 2, 110));
        // assert_eq!(obj.get_count(5, 100, 110), 1);

        let mut obj = Router::new(2);
        assert!(obj.add_packet(1, 4, 1));
        assert!(obj.add_packet(5, 4, 1));
        assert!(!obj.add_packet(1, 4, 1));
        assert_eq!(obj.get_count(4, 1, 1), 2);
        assert_eq!(obj.forward_packet(), vec![1, 4, 1]);
        assert_eq!(obj.get_count(4, 1, 1), 1);
    }
}
