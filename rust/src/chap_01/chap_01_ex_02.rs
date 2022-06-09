use std::cmp::Ordering::{Less, Greater};

fn get_smalest_window_to_sort<T>(arr:&[T])->(usize, usize)
where
    T: PartialOrd
{
    if arr.is_empty() {
        return (0,0);
    } else {
        let (_, right_index, _) = arr.iter().fold((0,0,&arr[0]), |(i,right_i,m), e| match e.partial_cmp(m) {
            Some(Less) => (i+1, i, m),
            _ => (i+1, right_i, e),
        } );
        let (_, left_index, _) = arr.iter().rev().fold((0,0,&arr[arr.len()-1]), |(i, left_i, m), e| match e.partial_cmp(m) {
            Some(Greater) => (i+1, i, m),
            _ => (i+1, left_i, e)
        });

        return (arr.len() - left_index - 1, right_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_to_sort(){
        assert_eq!((9,0), get_smalest_window_to_sort(&[0,1,2,3,4,5,6,7,8,9,]))
    }

    #[test]
    fn all_to_sort(){
        assert_eq!((0,9), get_smalest_window_to_sort(&[9,1,2,3,4,5,6,7,8,0,]))
    }

    #[test]
    fn sort_3_to_7(){
        assert_eq!((3,7), get_smalest_window_to_sort(&[0,1,2,7,4,5,6,3,8,9,]))

    }
}