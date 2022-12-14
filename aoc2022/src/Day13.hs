module Day13 (Nest, solve, more) where

import Data.List (elemIndex, sort)
import Data.Maybe
import GHC.Read
import Text.ParserCombinators.ReadPrec

data Nest = Int Int | Nil | Cons Nest Nest
  deriving (Show, Eq)

fromList :: [Nest] -> Nest
fromList = foldr Cons Nil

-- wow ok that's really clean, gotta love parsers
instance Read Nest where
  readPrec = (Int <$> readPrec) <++ (fromList <$> readPrec)

instance Ord Nest where
  Int a <= Int b = a <= b
  Nil <= Nil = True
  Nil <= Cons _ _ = True
  Cons _ _ <= Nil = False
  Cons a x <= Cons b y = (a, x) <= (b, y)
  Int a <= y = Cons (Int a) Nil <= y
  x <= Int b = x <= Cons (Int b) Nil

solve :: [(Nest, Nest)] -> Int
solve = sum . (fst <$>) . filter (uncurry (<=) . snd) . zip [1 ..]

separator :: Int -> Nest
separator n = Cons (Cons (Int n) Nil) Nil

more :: [Nest] -> Int
more xs = i * j
  where
    ys = sort (separator 2 : separator 6 : xs)
    i = fromJust (elemIndex (separator 2) ys) + 1
    j = fromJust (elemIndex (separator 6) ys) + 1
