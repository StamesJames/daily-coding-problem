module Haskell.Chap01.Chap01Ex01 where

-- Exercise: Calculate the for a Given List of Elements a List with the Products of all Elements except the one at that index

-- Solution: for Elements that are Fractional in O(n)
-- We build the product of all numbers in prod, then with map we divide prod at every index with the given element at that index. Whats Left is prod/(input !! i) for every i so the product of the remaining Elements
prods input = map (prod /) input where
    prod = product input


-- Solution: for Elements that aren't necessary Fractional in O(n)
-- we build the Produkt of of all Elements from the left to the index. 
prodsLeft input = 1:h where h = zipWith (*) (1:h) input
-- we build the Produkt of all Elements from the right to the index
prodsRight input = drop 1 $ reverse $ 1:h where h =  zipWith (*) (1:h) (reverse input)

-- with the products from the left and from the right we now can compute the Product of the Elements without one, be multiplying the left product till that element, and the right product till that element
prodsWithoutDiv input = zipWith (*) (prodsLeft input) (prodsRight input)