module Haskell.Chap01.Chap_01_Ex_04 where

import Haskell.Utils.AvlTree as AvlTree

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

-- This should now work in O(n log n) because the AvlTree.insert function should be in O(log n)
findNumberOfSmallerElementsToTheRightUsingAvlTree :: Ord a => [a] -> [Integer]
findNumberOfSmallerElementsToTheRightUsingAvlTree ls = fst $
    foldr 
        (\elem (counts, tree) -> let 
            (index, _ ,newTree) = AvlTree.insert elem tree
            newXs = index : counts
            in (newXs, newTree)) 
        ([], AvlTree.Leaf) 
        ls

test1 = findNumberOfSmallerElementsToTheRight [3,4,9,6,1]
test2 = findNumberOfSmallerElementsToTheRightUsingAvlTree [3,4,9,6,1]
