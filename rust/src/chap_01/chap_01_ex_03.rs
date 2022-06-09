use std::ops::{Mul, Div, Add};
use num::Zero;


// calculateMaxSubarraySum l = fst $ foldl f (0,0) l where 
//     f (max, maxLastElem) x = (newMax, newMaxLastElem) where
//         newMaxLastElem = if maxLastElem + x > x then maxLastElem + x else x 
//         newMax = if newMaxLastElem > max then newMaxLastElem else max

// calculateMaxSubarraySumWrap l = case foldl f ((0,0),(0,0)) (l++l) of tt@((_,m),_) -> m
//     where 
//         f t@(tMax@(cElemMax, max), tMaxLastElem@(cElemMaxLastElem, maxLastElem)) x 
//             | cElemMax < length l && cElemMaxLastElem < length l = (newTMax, newTMaxLastElem) 
//             | otherwise = t
//                 where
//                     newTMaxLastElem@(cNewMaxLastElem, newMaxLastElem) = if maxLastElem + x >= x then (cElemMaxLastElem + 1, maxLastElem + x) else (1, x) 
//                     newTMax@(cNewMax, newMax) = if newMaxLastElem > max then newTMaxLastElem else tMax


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