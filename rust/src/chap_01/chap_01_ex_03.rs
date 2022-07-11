use std::ops::{Mul, Div, Add};
use num::Zero;

// Exercise: Calculate the maximum Sum of an subarry. With and without the possibility of the subarray to wrap around

// Solution: with wrapping around in O(n)
// the same as without wrapping, but we double the list, and keep track of how much elements we already have summed, so that we don't take more then the whole list 
fn calculate_max_subarray_sum_wrap<T>(arr: &[T]) -> T
where
for<'a> &'a T : Add<&'a T, Output = T> + Ord, 
T: Clone + Zero,
{
    let mut count_max_last_elem = 0;
    let mut max = T::zero();
    let mut max_last_elem = T::zero();
    
    for elem in arr.iter().chain(arr.iter()) {
        let new_max_last_elem = if &(&max_last_elem + elem) > elem {count_max_last_elem += 1; &max_last_elem + elem} else {count_max_last_elem = 1; elem.clone()};
        let new_max = if &new_max_last_elem > &max {new_max_last_elem.clone()} else {max};
        
        max = new_max;
        max_last_elem = new_max_last_elem;
        if count_max_last_elem >= arr.len() {
            break;
        }
    }
    return max;
}

// Solution: without wrapping around in O(n)
// we iterate over the array list and keep track of the so biggest sum found so far as well as the maximal sum that includes the last seen element. The new maximal sum is then either the old maximal sum or the current element with summed with the maximal sum of the last element if its bigger then 0
fn calculate_max_subarray_sum<T>(arr: &[T]) -> T
where
    for<'a> &'a T : Add<&'a T, Output = T> + Ord, 
    T: Clone + Zero,
{
    fn f<T>((max, max_last_elem):(T,T), elem:&T) -> (T,T)
    where
        for<'a> &'a T : Add<&'a T, Output = T> + Clone + Ord,
        T: Clone,
    {
        let new_max_last_elem = if &(&max_last_elem + elem) > elem {&max_last_elem + elem} else {elem.clone()};
        let new_max = if &new_max_last_elem > &max {new_max_last_elem.clone()} else {max};
        return (new_max, new_max_last_elem);
    }
    return arr.iter().fold((T::zero(),T::zero()), f).0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_wrap(){
        assert_eq!(7, calculate_max_subarray_sum(&[1,1,1,-5,1,1,1,-5,1,1,1,1,1,1,1]))
    }

    #[test]
    fn with_wrap(){
        assert_eq!(10, calculate_max_subarray_sum_wrap(&[1,1,1,-5,1,1,1,-5,1,1,1,1,1,1,1]))
    }

}