use std::{collections::HashMap};

use unicode_segmentation::UnicodeSegmentation;



fn find_palindrome_pairs_brute_force(words: &[&str]) -> Vec<(usize, usize)> {
    let all_pairs = 
        words.iter().enumerate()
            .flat_map(
                |(index_1, word_1)| 
                    words.iter().enumerate()
                    .map(move |(index_2, word_2)| ((index_1, index_2), word_1.to_string() + word_2))
                    // If I collect this map, the second closure dosn't need to be move. I don't understand why
                    // .collect::<Vec<((usize, usize), String)>>()
            ).collect::<Vec<((usize, usize), String)>>();
    return all_pairs.iter()
        .filter(|(_,word)| is_palindrome(&word))
        .map(|tup| tup.0)
        .collect();
}


fn is_palindrome(word: &str) -> bool {
    let graphemes_forword = word.graphemes(true).take((word.len() / 2) + 1);
    let graphemes_backwardds = word.graphemes(true).rev().take((word.len() / 2) + 1);
    for (symbol_1,symbol_2) in Iterator::zip(graphemes_forword, graphemes_backwardds) {
            if symbol_1 != symbol_2 {
                return false;
            }
    }
    return true;
}

fn find_palindrome_pairs_with_map(words: &[&str]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut word_index_map = HashMap::new();
    words.iter().enumerate().for_each(|(i,elem)| {word_index_map.insert(*elem, i);});

    for (index,word) in words.iter().enumerate() {
        let len = word.len();
        for i in 0..len+1 {
            let prefix = &word[0..i];
            let suffix = &word[i..len];
            if is_palindrome(suffix) {
                let prefix_reversed = &prefix.graphemes(true).rev().collect::<String>() as &str;
                if let Some(j) = word_index_map.get( &prefix_reversed) {
                    result.push((index, *j));
                }
            }
        }
    }

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_edoc_da_d_brute_force(){
        let example = ["code", "edoc", "da", "d"];
        let solution = [(0,1), (1,0), (2,3), (3,3)];

        println!("{:?}", find_palindrome_pairs_brute_force(&example));

        assert_eq!(solution, find_palindrome_pairs_brute_force(&example).as_slice());
    }

    #[test]
    fn test_code_edoc_da_d_with_map(){
        let example = ["code", "edoc", "da", "d"];
        let solution = [(0,1), (1,0), (2,3), (3,3)];

        println!("{:?}", find_palindrome_pairs_with_map(&example));

        assert_eq!(solution, find_palindrome_pairs_with_map(&example).as_slice());
    }
}
