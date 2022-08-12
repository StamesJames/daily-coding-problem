module Haskell.Chap02.Chap02Ex02 where
import Haskell.Utils.Utils (enumerate)


-- Exercise: Find all pairs of indices so the concatinated strings form a Palindrom

example = ["code", "edoc", "da", "d"]

solution = [(0,1), (1,0), (2,3)]


isPalindrom w = w == reverse w

allPairsEnumerated ws = [(x,y) | x <- enumerate ws, y <- enumerate ws]

findPalindromPairs ws = map (\((i,x),(j,y)) -> (i,j) ) ( filter (\((i,x),(j,y)) -> isPalindrom $ x ++ y) (allPairsEnumerated ws) )
