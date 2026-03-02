module Utils where

import WeightedList


__ = wlist @String @Int []

tl = [(2, "sup"), (3, "nova"), (5, "shard")]

wl :: WeightedList String Int
wl = wlist tl


tll :: Int -> [(Int, String)]
tll n = map (\i -> (i, show i)) [1..n]

wll :: Int -> WeightedList String Int
wll n = wlist $ map (\i -> (i, show i)) [1..n]
