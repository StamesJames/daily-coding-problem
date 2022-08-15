module Haskell.Chap02.Chap02Ex02 where
import Haskell.Utils.Utils (enumerate)
import Control.Monad (guard)
import qualified Data.Map as Map



-- Exercise: Find all pairs of indices so the concatenated strings form a Palindrom

-- Solution: Because we build all pairs of elements and check for every concatenated string if it's a palindrome we our algorithm is in O(n^2 * w) where n is the count of strings and w is the length of the longest string. 

example = ["code", "edoc", "da", "d"]

solution = [(0,1), (1,0), (2,3)]


isPalindrom w = w == reverse w

allPairsEnumerated ws = [(x,y) | x <- enumerate ws, y <- enumerate ws]

findPalindromPairs ws = map (\((i,x),(j,y)) -> (i,j) ) ( filter (\((i,x),(j,y)) -> isPalindrom $ x ++ y) (allPairsEnumerated ws) )

findPalindromPairsMonadStyle ws = do 
    ((i,x),(j,y)) <- allPairsEnumerated ws
    guard $ isPalindrom (x ++ y)
    return (i,j)

-- Solution: another Solution is to look for every word if its suffix is a palindrome and then if the reversed prefix is in our list. If we would use a constant lookup Map we would get a an algorithm in O(n * w^2)

putInMap ws = foldl (\m (i, elem) -> Map.insert elem i m) Map.empty (enumerate ws)

allSuffixes [] = [""]
allSuffixes w@(x:xs) = w : allSuffixes xs 

-- I've written this function in this way just for practice. for a faster solution we could get rid of some reverses, because in the function findPalindromPairsQuadratInWord we use the words reversed anyways 
allPrefixes = reverse . map reverse . allSuffixes . reverse

findPalindromPairsQuadratInWord ws = do
    let m = putInMap ws
    w <- ws
    (prefRev, suf) <- zip (map reverse $ allPrefixes w) (allSuffixes w)
    guard $ isPalindrom suf && Map.member prefRev m
    return (m Map.! w, m Map.! prefRev)
