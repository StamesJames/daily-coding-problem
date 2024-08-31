use std::vec;

use crate::utils::avl_tree::AvlTree;

pub fn find_number_of_smaller_elems_at_the_right<T>(arr: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let mut result = vec![0; arr.len()];
    let mut avl_tree = AvlTree::new();
    for (i, elem) in arr.iter().enumerate().rev() {
        let (index, _) = avl_tree.insert(elem);
        result[i] = index;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_numbers_to_right_3_4_9_6_1() {
        assert_eq!(
            &[1, 1, 2, 1, 0],
            find_number_of_smaller_elems_at_the_right(&[3, 4, 9, 6, 1]).as_slice()
        )
    }
    #[test]
    fn test_smaller_numbers_to_right_1_1_1_1_1() {
        assert_eq!(
            &[0, 0, 0, 0, 0],
            find_number_of_smaller_elems_at_the_right(&[1, 1, 1, 1, 1]).as_slice()
        )
    }
    #[test]
    fn test_smaller_numbers_to_right_1_2_3_4_5_6() {
        assert_eq!(
            &[0, 0, 0, 0, 0, 0],
            find_number_of_smaller_elems_at_the_right(&[1, 2, 3, 4, 5, 6]).as_slice()
        )
    }
    #[test]
    fn test_smaller_numbers_to_right_6_5_4_3_2_1() {
        assert_eq!(
            &[5, 4, 3, 2, 1, 0],
            find_number_of_smaller_elems_at_the_right(&[6, 5, 4, 3, 2, 1]).as_slice()
        )
    }
}
