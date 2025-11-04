module WeightedList where

import Data.List


---------------------------------------------------------------------

type WeightedList v w = [WeightedItem v w]


data WeightedItem v w = WeightedItem
    { value :: v
    , weight :: w
    , c_weight :: w  -- cumulative weight for binary search
    }
  deriving (Show)

instance (Eq v, Eq w) => Eq (WeightedItem v w) where
  item == item'
    = (
      value item == value item'
      && weight item == weight item'
    )


---------------------------------------------------------------------

{-|
Construct a list of `WeightedItem`s from the provided (weight, value) pairs.
-}
newWeightedList :: forall v w . Num w
                => [(w, v)]
                -> WeightedList v w

newWeightedList [] = []

newWeightedList ((fst_weight, fst_value):items)
    = scanl prep item' items
  where
    item' :: WeightedItem v w
    item' = WeightedItem { value = fst_value, weight = fst_weight, c_weight = 0 }

    prep :: WeightedItem v w
         -> (w, v)
         -> WeightedItem v w
    prep acc (weight', value')
      = WeightedItem {
          value = value',
          weight = weight',
          c_weight = weight' + c_weight acc
        }


{- |
Count the total number of items in a `WeightedList`.
-}
total_values :: WeightedList v w -> Int
total_values = length

{- |
Sum the total weights of all items in a `WeightedList`.
-}
total_weights :: (Num w) => WeightedList v w -> w
total_weights = foldl' (\t item -> t + weight item) 0

total_weights' :: (Num w) => WeightedList v w -> w
total_weights' = sum . map weight

{- |
Get a list of the values of all items in a `WeightedList`.
-}
values :: WeightedList v w -> [v]
values = map value

{- |
Get a list of the weights of all items in a `WeightedList`.
-}
weights :: WeightedList v w -> [w]
weights = map weight

{- |
Get the raw representation of a `WeightedList` in (weight, value) pairs.

This satisfies the axiom `newWeightedList (raw (newWeightedList list)) == newWeightedList list`.
-}
raw :: WeightedList v w -> [(w, v)]
raw = map (\item -> (weight item, value item))

{- |
Get the raw representation of a `WeightedList` in (value, weight) pairs.
-}
raw' :: WeightedList v w -> [(v, w)]
raw' = map (\item -> (value item, weight item))


{- |
Get the item of a `WeightedList` at a weighted index.

If a negative index is provided, access starts from the end of the list (where the last item is at index `-1`).
 
Note that this has $O(n)$ time complexity.
-}
get :: forall v w . (Num w, Ord w)
    => WeightedList v w
    -> w
    -> WeightedItem v w

get [] _ = error "Cannot access an empty WeightedList"

get list i
    -- | i < 0     = get_r list i (-1)
    | otherwise = get' list i 0
  where
    get' :: [WeightedItem v w]
         -> w
         -> w
         -> WeightedItem v w
    
    get' (item:items) i' t
        | t' > i'   = item
        | otherwise = get' items i' t'
      where
        t' = t + (weight item)
    
    get_r :: [WeightedItem v w]
          -> w
          -> w
          -> WeightedItem v w
    
    get_r (item:items) i' t'
      = undefined
