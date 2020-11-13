use std::collections::{HashMap, HashSet};
#[derive(Default)]
struct Twitter {
    tweets: Vec<Vec<i32>>,
    follows: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push(vec![user_id, tweet_id]);
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.tweets
            .iter()
            .rev()
            .filter(|t| {
                t[0] == user_id
                    || self.follows.contains_key(&user_id) && self.follows[&user_id].contains(&t[0])
            })
            .take(10)
            .map(|v| v[1])
            .collect()
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        (*self.follows.entry(follower_id).or_insert(HashSet::new())).insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(follows) = self.follows.get_mut(&follower_id) {
            follows.remove(&followee_id);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0355() {
        let mut obj = Twitter::new();
        obj.post_tweet(1, 5);
        assert_eq!(vec![5], obj.get_news_feed(1));
        obj.follow(1, 2);
        obj.post_tweet(2, 6);
        assert_eq!(vec![6, 5], obj.get_news_feed(1));
        obj.unfollow(1, 2);
        assert_eq!(vec![5], obj.get_news_feed(1));

        let mut obj = Twitter::new();
        obj.post_tweet(1, 4);
        obj.post_tweet(2, 5);
        obj.unfollow(1, 2);
        assert_eq!(vec![4], obj.get_news_feed(1));
    }
}
