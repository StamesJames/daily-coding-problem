module Chap_01_Ex_01 where

import Text.XHtml (input)

prods input = map (product input /) input

prodsLeft input = 1:h where h = zipWith (*) (1:h) input
prodsRight input = drop 1 $ reverse $ 1:h where h =  zipWith (*) (1:h) (reverse input)

prodsWithoutDiv input = zipWith (*) (prodsLeft input) (prodsRight input)