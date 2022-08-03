module Haskell.Chap01.Chap01Ex02 where

-- Exercise: for a List calculate the smallest Window form where to where it has to be sorted, so that the whole list is sorted

-- Solution: in O(n)
-- we iterate once from the left and once from the right. while Iterating we keep track of the smallest or largest Element so far. How far this Element has to travel is the index to which or from which the List has to be sorted. It's like one Bubblesort iteration from the right and one from the left
getSmalestWindowToSort [] = (0,0)
getSmalestWindowToSort l = (leftIndex l, rightIndex l) where
    rightIndex [] = 0
    rightIndex l@(x:xs) = case foldl (\(i, right_i, m) e -> if e < m then (i+1, i, m) else (i+1, right_i, e)) (0, 0, x) l of (_, ri, _) -> ri
    leftIndex [] = 0
    leftIndex l = case (reverse l) of 
        [] -> 0
        l'@(x:xs) -> case foldl (\(i, left_i, m) e -> if e > m then (i+1, i, m) else (i+1, left_i, e) ) (0,0,x) l' of (length, li, _) -> length - li - 1