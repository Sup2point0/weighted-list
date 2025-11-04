module WeightedList where

import Data.List


---------------------------------------------------------------------

type WeightedList v w = [WeightedItem v w]


data WeightedItem v w = WeightedItem
    { value :: v
    , weight :: w
    }
  deriving (Eq, Show)


-- instance (Show v, Show w) => Show WeightedItem v w where
--   show (WeightedItem weight value)
--     = "(" ++ show value ++ ", " ++ show weight ++ ")"


---------------------------------------------------------------------

newWeightedList :: forall v w . Num w
                => [(w, v)]
                -> WeightedList v w

newWeightedList items
    = map sanitise items
  where
    sanitise :: (w, v) -> WeightedItem v w
    sanitise (weight, value) = WeightedItem { value = value, weight = weight }


-- | Count the total number of items in a `WeightedList`.
total_values :: WeightedList v w -> Int

total_values
  = length

-- | Sum the total weights of all items in a `WeightedList`.
total_weights :: (Num w) => WeightedList v w -> w

total_weights list
  = sum (map weight list)
  -- = foldl' (\t item -> t + weight item) 0 list

-- | Get a list of the values of all items in a `WeightedList`.
values :: WeightedList v w -> [v]

values list
  = map value list

-- | Get a list of the weights of all items in a `WeightedList`.
weights :: WeightedList v w -> [w]

weights list
  = map weight list

-- | Get the raw representation of a `WeightedList` in (weight, value) pairs.
-- |
-- | This satisfies the axiom `newWeightedList (raw (newWeightedList list)) == newWeightedList list`.
raw :: WeightedList v w -> [(w, v)]

raw list
  = map (\item -> (weight item, value item)) list

-- | Get the raw representation of a `WeightedList` in (value, weight) pairs.
raw' :: WeightedList v w -> [(v, w)]
raw' list
  = map (\item -> (value item, weight item)) list


-- | Get the item of a `WeightedList` at a weighted index.
get :: forall v w . (Num w, Ord w)
    => WeightedList v w
    -> w
    -> WeightedItem v w

get list i
    = get' list i 0
  where
    get' :: [WeightedItem v w]
         -> w
         -> w
         -> WeightedItem v w

    get' [] _ _
      = error "Cannot access an empty WeightedList"
    
    get' (item:items) i' t
        | t' > i'   = item
        | otherwise = get' items i' t'
      where
        t' = t + (weight item)
