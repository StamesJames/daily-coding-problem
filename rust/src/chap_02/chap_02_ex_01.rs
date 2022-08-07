use std::{collections::HashMap, hash::Hash, isize};

use queues::{Queue, IsQueue};

#[derive(Debug)]
enum Counter<K> 
where
    K: Eq + Hash
    {
        Counter(HashMap<K,isize>),
    } 
    
impl<K> Counter<K> 
where
    K: Eq + Hash + Clone
{
    fn add_count(&mut self, elem: &K, count: isize){
        let Self::Counter(map) = self;
        let old_val = map.get(elem);
        match old_val {
            Some(old_count) => {
                let new_val = old_count + count;
                if new_val == 0 {
                    map.remove(elem);
                }else {
                    map.insert(elem.clone(), new_val);
                }
            },
            None => {map.insert(elem.clone(), count);},
        };
    }

    fn increment(&mut self, elem: &K){
        self.add_count(elem, 1);
    }
    fn decrement(&mut self, elem: &K){
        self.add_count(elem, -1);
    }
    fn empty(&self) -> bool {
        let Counter::Counter(map) = self;
        return map.is_empty();
    }
    fn new() -> Counter<K>{
        return Counter::Counter(HashMap::new());
    }
}

impl Counter<char> {
    fn from_string(str: &str) -> Counter<char>{
        let mut counter = Counter::new();
        for char in str.chars() {
            counter.increment(&char);
        }
        return counter;
    }
}

pub fn find_anagram_indices(w: &str, s: &str) -> Vec<usize> {
    let mut count_w = Counter::from_string(&w);
    let mut result = Vec::new();
    let mut window = Queue::new();
    println!("{count_w:?}");
    if w.len() > s.len() {return vec![];}
    for (i, char) in s.chars().enumerate() {
        if i < w.len() {
            window.add(char);
            count_w.decrement(&char);
        } else {
            if let Ok(old_char) = window.remove() {
                count_w.increment(&old_char);
                count_w.decrement(&char);
                window.add(char);
            } else {
                panic!("something went wrong");
            }
        }
        if count_w.empty() {
            result.push(i-(w.len()-1));
        }
        println!("{count_w:?}");
    }

    return result;
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab_abxaba(){
        assert_eq!(vec![0,3,4,], find_anagram_indices("ab", "abxaba"));
    }
    #[test]
    fn test_abcd_0123acbda9(){
        assert_eq!(vec![4,5,], find_anagram_indices("abcd", "0123abcda9"));
    }
    #[test]
    fn test_abcd_00acbda000ab00cd00dcba(){
        assert_eq!(vec![2,3,18,], find_anagram_indices("abcd", "00acbda000ab00cd00dcba"));
    }


}