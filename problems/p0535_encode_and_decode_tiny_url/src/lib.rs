use rand::{thread_rng, Rng};
use std::{cell::RefCell, collections::HashMap};

struct Codec {
    encode_map: RefCell<HashMap<String, String>>,
    decode_map: RefCell<HashMap<String, String>>,
    chars: Vec<char>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    const URL_PREFIX: &'static str = "http://tinyurl.com/";

    fn new() -> Self {
        Self {
            encode_map: Default::default(),
            decode_map: Default::default(),
            chars: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .collect(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        let mut rng = thread_rng();
        let mut encoded = String::new();
        while encoded.is_empty() || self.encode_map.borrow().contains_key(&encoded) {
            encoded = Self::URL_PREFIX.to_string();
            (0..7).for_each(|_| encoded.push(self.chars[rng.gen_range(0, 62)]));
        }

        self.decode_map
            .borrow_mut()
            .insert(encoded.clone(), longURL.clone());
        self.encode_map
            .borrow_mut()
            .insert(longURL, encoded.clone());

        encoded
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        self.decode_map.borrow().get(&shortURL).unwrap().to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0535() {
        let obj = Codec::new();
        let e = obj.encode("http://www.google.com/".to_string());
        println!("{}", e);
        let d = obj.decode(e);
        assert_eq!(d, "http://www.google.com/".to_string());
    }
}
