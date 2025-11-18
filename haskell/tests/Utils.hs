module Utils where

import WeightedList


__ = newWeightedList @String @Int []

l  = [(2, "sup"), (3, "nova"), (7, "shard")]

wl :: WeightedList String Int
wl = newWeightedList l

wll :: Int -> WeightedList String Int
wll n = newWeightedList $ map item [1..n]
  where
    item :: Int -> (Int, String)
    item i = (i, show i)
