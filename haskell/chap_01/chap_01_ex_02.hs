module Chap_01_Ex_02 where


sample = [0,1,2,3,4,5,6,7,8,9]

getSmalestWindowToSort [] = (0,0)
getSmalestWindowToSort l = (leftIndex l, rightIndex l) where
    rightIndex [] = 0
    rightIndex l@(x:xs) = case foldl (\(i, right_i, m) e -> if e < m then (i+1, i, m) else (i+1, right_i, e)) (0, 0, x) l of (_, ri, _) -> ri
    leftIndex [] = 0
    leftIndex l = case (reverse l) of 
        [] -> 0
        l'@(x:xs) -> case foldl (\(i, left_i, m) e -> if e > m then (i+1, i, m) else (i+1, left_i, e) ) (0,0,x) l' of (length, li, _) -> length - li - 1