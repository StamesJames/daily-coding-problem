module Haskell.Chap02.Chap02Ex01 where

import qualified Data.Map as Map
import qualified Haskell.Utils.Queue as Que
import Language.Haskell.TH (match)

-- Exercise: Given a Word w and another Word s, find all indices in s where a substring starts, that is a anagram of w 

w = "ab"
s = "abxaba"
result = [0,3,4]


-- for an anagram its only important which letters and how much of them there are. So we use a Map to count the letters of a given substring. We can imagine it like a search window we traverse over our string s and look if the counts of letters match those of w. 
-- to memorize this window we use a Queue Datastructure
-- This implementation uses haskells build in Map datastructure because this has a logarithmic lookup we need this solution is in O(n log n). with a more efficient Map it should be possible to make it a bit more efficient and with constant lookup Maps we get O(n) where n is the size of s
-- One thing to notice, this solution gives back the result in reversed order.
findAnagramStartingIndices w s = 
    if length w > length s || length w == 0 then []
    else
    let 
        wLength = length w
        wMap = mapify w
    in 
        case foldl 
        (\(i, accString, accMap, acc) elem ->
            if Que.queCount accString < wLength then
                let
                    newIndex = i + 1
                    newAccString = Que.enqueue elem accString
                    newAccMap = reduceCount elem accMap
                    newAcc = if Map.null newAccMap then (i-wLength+1):acc else acc
                in
                    (newIndex, newAccString, newAccMap, newAcc)
            else
                case Que.dequeue accString of 
                    Just (x, q) -> 
                        let 
                            newIndex = i + 1
                            newAccString = Que.enqueue elem q
                            newAccMap = addCount x (reduceCount elem accMap)
                            newAcc = if Map.null newAccMap then i-wLength+1:acc else acc
                        in 
                            (newIndex, newAccString, newAccMap, newAcc)
                    Nothing -> (0, Que.emptyQueue, Map.empty, [-1])

        ) 
        (0, Que.emptyQueue, wMap, []) 
        s
        of (_,_,_,acc) -> acc


mapify :: Ord a => [a] -> Map.Map a Integer
mapify = foldl (flip addCount) Map.empty

addCount :: Ord a => a -> Map.Map a Integer -> Map.Map a Integer
addCount x m = 
    if Map.member x m then 
        if newVal == 0 then 
            Map.delete x m 
        else 
            Map.insert x newVal m
    else
        Map.insert x 1 m
    where 
        newVal = (m Map.! x) + 1

reduceCount :: Ord a => a -> Map.Map a Integer -> Map.Map a Integer
reduceCount x m = 
    if Map.member x m then 
        if newVal == 0 then 
            Map.delete x m 
        else 
            Map.insert x newVal m
    else
        Map.insert x (-1) m
    where 
        newVal = (m Map.! x) - 1


-- A BruteForce Solution that is a bit easier to read


getAllSubstringsOfLength :: String -> Int -> [String]
getAllSubstringsOfLength s i = [take i (drop x s) | x <- [0..length s - i]]

enumerate l = snd $ foldr (\elem (i,acc) -> (i-1, (i,elem):acc)) (length l - 1, []) l

findAnagramStartingIndicesBruteForce :: String -> String -> [Int]
findAnagramStartingIndicesBruteForce w s = 
    let 
        countW = mapify w
        allSubstrings = enumerate (getAllSubstringsOfLength s (length w))
    in
        map fst (filter (\(i, str) -> mapify str == countW) allSubstrings)

