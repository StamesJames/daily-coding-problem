use crate::utils::counter::Counter;
use queues::{IsQueue, Queue};
use unicode_segmentation::UnicodeSegmentation;

pub fn find_anagram_indices(w: &str, s: &str) -> Vec<usize> {
    let mut count_w = Counter::from_string_grapheme(w);
    let mut result = Vec::new();
    let mut window = Queue::new();
    println!("{count_w:?}");
    if w.len() > s.len() {
        return vec![];
    }
    for (i, char) in s.graphemes(true).enumerate() {
        if i < w.len() {
            window.add(char).expect("queue broken");
            count_w.decrement(&char);
        } else if let Ok(old_char) = window.remove() {
            count_w.increment(&old_char);
            count_w.decrement(&char);
            window.add(char).expect("queue broken");
        } else {
            panic!("something went wrong");
        }
        if count_w.empty() {
            result.push(i - (w.len() - 1));
        }
        println!("{count_w:?}");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab_abxaba() {
        assert_eq!(vec![0, 3, 4,], find_anagram_indices("ab", "abxaba"));
    }
    #[test]
    fn test_abcd_0123acbda9() {
        assert_eq!(vec![4, 5,], find_anagram_indices("abcd", "0123abcda9"));
    }
    #[test]
    fn test_abcd_00acbda000ab00cd00dcba() {
        assert_eq!(
            vec![2, 3, 18,],
            find_anagram_indices("abcd", "00acbda000ab00cd00dcba")
        );
    }
}
