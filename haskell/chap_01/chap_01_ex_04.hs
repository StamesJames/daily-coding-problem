module Chap_01_Ex_04 where

-- Exercise: Calculate for every Element in a list, how many smaller elements are to the right of this element

    
-- This implementation is not very efficient because we can't perform binary Search on Lists, so it is in O(n)
-- with a proper BST implementation this should work in O(log n)
ordInsert a [] = (0,[a])
ordInsert a (x:xs) = if x > a then (0,a:x:xs) else case ordInsert a xs of (i,ls) -> (i+1,x:ls)

-- because we use the inefficient implementation of ordInsert this is in O(n^2) but with a proper BST this should work in O(n log n)
findNumberOfSmallerElementsToTheRight :: Ord a => [a] -> [Integer]
findNumberOfSmallerElementsToTheRight ls = result where
    lsRev = reverse ls
    result = fst $ foldl (\(indices, sorted) n -> case ordInsert n sorted of (i, newSorted) -> (i:indices, newSorted)) ([],[]) lsRev



test1 = findNumberOfSmallerElementsToTheRight [3,4,9,6,1]
