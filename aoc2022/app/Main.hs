module Main (main) where

import Day13

pairs :: [Day13.Nest] -> [(Day13.Nest, Day13.Nest)]
pairs (a : b : t) = (a, b) : pairs t
pairs _ = []

main :: IO ()
main = do
  s <- readFile "../input/day13/input.txt"
  let v = read <$> filter (/= []) (lines s)
  print $ Day13.solve $ pairs v
  print $ Day13.more v
