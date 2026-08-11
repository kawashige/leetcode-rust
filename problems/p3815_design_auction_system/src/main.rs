use std::collections::{BinaryHeap, HashMap};

struct AuctionSystem {
    item_bits: HashMap<i32, BinaryHeap<(i32, i32)>>,
    user_bits: HashMap<(i32, i32), i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionSystem {
    fn new() -> Self {
        Self {
            item_bits: HashMap::new(),
            user_bits: HashMap::new(),
        }
    }

    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        self.user_bits.insert((user_id, item_id), bid_amount);
        (*self.item_bits.entry(item_id).or_insert(BinaryHeap::new())).push((bid_amount, user_id));
    }

    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        self.add_bid(user_id, item_id, new_amount);
    }

    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        self.user_bits.remove(&(user_id, item_id));
    }

    fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
        if let Some(bids) = self.item_bits.get_mut(&item_id) {
            while let Some((bid_amount, user_id)) = bids.pop() {
                if self.user_bits.get(&(user_id, item_id)) == Some(&bid_amount) {
                    bids.push((bid_amount, user_id));
                    return user_id;
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
    fn test_3815() {
        let mut obj = AuctionSystem::new();
        obj.add_bid(1, 7, 5);
        obj.add_bid(2, 7, 6);
        assert_eq!(obj.get_highest_bidder(7), 2);
        obj.update_bid(1, 7, 8);
        assert_eq!(obj.get_highest_bidder(7), 1);
        obj.remove_bid(2, 7);
        assert_eq!(obj.get_highest_bidder(7), 1);
        assert_eq!(obj.get_highest_bidder(3), -1);
    }
}

fn main() {}
