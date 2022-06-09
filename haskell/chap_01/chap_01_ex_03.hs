module Chap_01_Ex_03 where


input1 = [34, -50, 42, 14, -5, 86]
answere1 = 137

input2 = [-5, -1, -8, -9]
answere2 = 0


calculateMaxSubarraySum l = fst $ foldl f (0,0) l where 
    f (max, maxLastElem) x = (newMax, newMaxLastElem) where
        newMaxLastElem = if maxLastElem + x > x then maxLastElem + x else x 
        newMax = if newMaxLastElem > max then newMaxLastElem else max

calculateMaxSubarraySumWrap l = case foldl f ((0,0),(0,0)) (l++l) of tt@((_,m),_) -> m
    where 
        f t@(tMax@(cElemMax, max), tMaxLastElem@(cElemMaxLastElem, maxLastElem)) x 
            | cElemMax < length l && cElemMaxLastElem < length l = (newTMax, newTMaxLastElem) 
            | otherwise = t
                where
                    newTMaxLastElem@(cNewMaxLastElem, newMaxLastElem) = if maxLastElem + x >= x then (cElemMaxLastElem + 1, maxLastElem + x) else (1, x) 
                    newTMax@(cNewMax, newMax) = if newMaxLastElem > max then newTMaxLastElem else tMax