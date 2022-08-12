module Haskell.Chap02.Chap02Ex02 where
import Haskell.Utils.Utils (enumerate)
import Control.Monad (guard)


-- Exercise: Find all pairs of indices so the concatinated strings form a Palindrom

-- Solution: Because we build all pairs of elementes and check for every concatinatet string if it's a palindrom we our algorithm is in O(n^2 * w) where n is the count of strings and w is the length of the longest string. 

example = ["code", "edoc", "da", "d"]

solution = [(0,1), (1,0), (2,3)]


isPalindrom w = w == reverse w

allPairsEnumerated ws = [(x,y) | x <- enumerate ws, y <- enumerate ws]

findPalindromPairs ws = map (\((i,x),(j,y)) -> (i,j) ) ( filter (\((i,x),(j,y)) -> isPalindrom $ x ++ y) (allPairsEnumerated ws) )

findPalindromPairsMonadStyle ws = do 
    ((i,x),(j,y)) <- allPairsEnumerated ws
    guard $ isPalindrom (x ++ y)
    return (i,j)


