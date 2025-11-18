module Utils where

import WeightedList


__ = newWeightedList @String @Int []

tl = [(2, "sup"), (3, "nova"), (7, "shard")]

wl :: WeightedList String Int
wl = newWeightedList tl


tll :: Int -> [(Int, String)]
tll n = map (\i -> (i, show i)) [1..n]

wll :: Int -> WeightedList String Int
wll n = newWeightedList $ map (\i -> (i, show i)) [1..n]
