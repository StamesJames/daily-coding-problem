use std::cmp::Ordering::{Greater, Less};

// Exercise: for a List calculate the smallest Window from where to where it has to be sorted, so that the whole list is sorted

// Solution: in O(n)
pub fn get_smallest_window_to_sort<T>(arr: &[T]) -> (usize, usize)
where
    T: PartialOrd,
{
    if arr.is_empty() {
        (0, 0)
    } else {
        // we calculate the right index by iterating over teh array from the left and keeping track of the biggest element so far. We know, when we find an element smaller then the currently biggest, that we at least have to push the biggest element to the right to this position. It's like one step in the Bubblesort algorithm.
        let (_, right_index, _) = arr.iter().fold((0, 0, &arr[0]), |(i, right_i, m), e| {
            match e.partial_cmp(m) {
                Some(Less) => (i + 1, i, m),
                _ => (i + 1, right_i, e),
            }
        });
        // The same as for the left index but in the other direction
        let (_, left_index, _) =
            arr.iter()
                .rev()
                .fold((0, 0, &arr[arr.len() - 1]), |(i, left_i, m), e| {
                    match e.partial_cmp(m) {
                        Some(Greater) => (i + 1, i, m),
                        _ => (i + 1, left_i, e),
                    }
                });
        // because of how we calculated the left index we have to calculate its reverse, because we used the reversed array
        (arr.len() - left_index - 1, right_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_to_sort() {
        assert_eq!(
            (9, 0),
            get_smallest_window_to_sort(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9,])
        )
    }

    #[test]
    fn all_to_sort() {
        assert_eq!(
            (0, 9),
            get_smallest_window_to_sort(&[9, 1, 2, 3, 4, 5, 6, 7, 8, 0,])
        )
    }

    #[test]
    fn sort_3_to_7() {
        assert_eq!(
            (3, 7),
            get_smallest_window_to_sort(&[0, 1, 2, 7, 4, 5, 6, 3, 8, 9,])
        )
    }
}
