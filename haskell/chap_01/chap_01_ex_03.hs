module Chap_01_Ex_03 where

-- Exercise: Calculate the maximum Sum of an subarry. With and without the possibility of the subarray to wrap around

-- Solution: without wrapping around in O(n)
-- we iterate over the array list and keep track of the so biggest sum found so far as well as the maximal sum that includes the last seen element. The new maximal sum is then either the old maximal sum or the current element with summed with the maximal sum of the last element if its bigger then 0
calculateMaxSubarraySum l = fst $ foldl f (0,0) l where 
    f (max, maxLastElem) x = (newMax, newMaxLastElem) where
        newMaxLastElem = if maxLastElem + x > x then maxLastElem + x else x 
        newMax = if newMaxLastElem > max then newMaxLastElem else max

-- Solution: without wrapping around in O(n)
-- the same as without wrapping, but we double the list, and keep track of how much elements we already have summed, so that we don't take more then the whole list 
calculateMaxSubarraySumWrap l = case foldl f ((0,0),(0,0)) (l++l) of tt@((_,m),_) -> m
    where 
        f t@(tMax@(cElemMax, max), tMaxLastElem@(cElemMaxLastElem, maxLastElem)) x 
            | cElemMax < length l && cElemMaxLastElem < length l = (newTMax, newTMaxLastElem) 
            | otherwise = t
                where
                    newTMaxLastElem@(cNewMaxLastElem, newMaxLastElem) = if maxLastElem + x >= x then (cElemMaxLastElem + 1, maxLastElem + x) else (1, x) 
                    newTMax@(cNewMax, newMax) = if newMaxLastElem > max then newTMaxLastElem else tMax