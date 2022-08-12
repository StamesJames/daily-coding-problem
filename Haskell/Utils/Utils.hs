module Haskell.Utils.Utils(enumerate) where

enumerate l = reverse $ snd $ foldl (\(i,acc) elem -> (i+1, (i,elem):acc)) (0, []) l
