use std::{collections::HashMap, hash::Hash, isize};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum Counter<K>
where
    K: Eq + Hash,
{
    Counter(HashMap<K, isize>),
}

impl<K> Counter<K>
where
    K: Eq + Hash + Clone,
{
    pub fn add_count(&mut self, elem: &K, count: isize) {
        let Self::Counter(map) = self;
        let old_val = map.get(elem);
        match old_val {
            Some(old_count) => {
                let new_val = old_count + count;
                if new_val == 0 {
                    map.remove(elem);
                } else {
                    map.insert(elem.clone(), new_val);
                }
            }
            None => {
                map.insert(elem.clone(), count);
            }
        };
    }

    pub fn increment(&mut self, elem: &K) {
        self.add_count(elem, 1);
    }
    pub fn decrement(&mut self, elem: &K) {
        self.add_count(elem, -1);
    }
    pub fn empty(&self) -> bool {
        let Counter::Counter(map) = self;
        map.is_empty()
    }
    pub fn new() -> Counter<K> {
        Counter::Counter(HashMap::new())
    }
}

impl<K> Default for Counter<K>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl Counter<char> {
    pub fn from_string(str: &str) -> Counter<char> {
        let mut counter = Counter::new();
        for char in str.chars() {
            counter.increment(&char);
        }
        counter
    }
}

impl Counter<&str> {
    pub fn from_string_grapheme(str: &str) -> Counter<&str> {
        let mut counter: Counter<&str> = Counter::new();
        for grapheme in str.graphemes(true) {
            counter.increment(&grapheme);
        }
        counter
    }
}
