use std::ops::{Mul, Div};

pub fn prod_from_elems<T>(arr: &[T])-> Vec<T>
where 
    for<'a> T : Mul<&'a T, Output = T> + Clone,
    for<'a> &'a T: Div<&'a T, Output = T>
{
    if arr.len() == 0 {
        return vec![];
    }else {
        let initial = arr[0].clone();
        let prod = arr.iter().skip(1).fold(initial, |x,y| x* y);
        return arr.iter().map(|e| &prod / e).collect();
    }
}

pub fn prod_from_elems_without_div<T>(arr: &[T])-> Vec<T>
where
    for<'a> &'a T : Mul<&'a T, Output = T>,
    T : Clone
{
    if arr.len() == 0 {
        return vec![];
    } else {
        let mut left_acc = arr[0].clone();
        let mut right_acc = arr[arr.len()-1].clone();
        let mut left_prods = vec![];
        let mut right_prods = vec![];
        arr.iter().skip(1).for_each(|e| { left_prods.push(left_acc.clone()); left_acc = &left_acc * e;});
        arr.iter().rev().skip(1).for_each(|e| { right_prods.push(right_acc.clone()); right_acc = &right_acc * e;});
        let mut result = vec![];
        result.push(right_prods[right_prods.len()-1].clone());
        for i in 1..left_prods.len() {
            result.push(&left_prods[i-1] * &right_prods[right_prods.len()-1-i ])
        }
        result.push(left_prods[left_prods.len()-1].clone());
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_div(){
        assert_eq!(vec![120.,60.,40.,30.,24.,], prod_from_elems_without_div(&[1.,2.,3.,4.,5.,]))
    }

    #[test]
    fn with_div(){
        assert_eq!(vec![120.,60.,40.,30.,24.,], prod_from_elems(&[1.,2.,3.,4.,5.,]))

    }
}