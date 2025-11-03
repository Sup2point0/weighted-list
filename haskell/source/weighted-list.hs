module WeightedList where

import Data.List


---------------------------------------------------------------------

data WeightedItem v w = WeightedItem
    { value :: v
    , weight :: w
    }
  deriving (Eq)


instance (Show v, Show w) => Show (WeightedItem v w) where
  show (WeightedItem weight value)
    = "(" ++ show value ++ ", " ++ show weight ++ ")"


---------------------------------------------------------------------

newtype WeightedList v w = WeightedList [WeightedItem v w]
  deriving (Show)


newWeightedList :: forall v w . (Num w) => [(w, v)] -> WeightedList v w
newWeightedList items
    = WeightedList (map sanitise items)
  where
    sanitise :: (w, v) -> WeightedItem v w
    sanitise (weight, value) = WeightedItem { value = value, weight = weight }


instance (Eq v, Eq w) => Eq (WeightedList v w) where
  WeightedList [] == WeightedList [] = True
  WeightedList [] == WeightedList _  = False
  WeightedList _  == WeightedList [] = False
  WeightedList prot == WeightedList deut = (prot == deut)


-- | Count the total number of items in a `WeightedList`.
total_values :: (WeightedList v w) -> Int
total_values (WeightedList list)
  = length list

-- | Sum the total weights of all items in a `WeightedList`.
total_weights :: (Num w) => (WeightedList v w) -> w
total_weights (WeightedList list)
  -- = foldl' (\ t item -> t + weight item) 0 list
  = sum (map (\ item -> weight item) list)

-- | Get a list of the values of all items in a `WeightedList`.
values :: (WeightedList v w) -> [v]
values (WeightedList list)
  = map (\ item -> value item) list

-- | Get a list of the weights of all items in a `WeightedList`.
weights :: (WeightedList v w) -> [w]
weights (WeightedList list)
  = map (\ item -> weight item) list

-- | Get the raw representation of a `WeightedList` in (weight, value) pairs.
-- |
-- | This satisfies the axiom `WeightedList (raw WeightedList list) == WeightedList list`.
raw :: (WeightedList v w) -> [(w, v)]
raw (WeightedList list)
  = map (\ item -> (weight item, value  item)) list

-- | Get the raw representation of a `WeightedList` in (value, weight) pairs.
raw' :: (WeightedList v w) -> [(v, w)]
raw' (WeightedList list)
  = map (\ item -> (value item, weight item)) list


-- | Get the item of a `WeightedList` at a weighted index.
get :: forall v w . (Num w, Ord w)
    => (WeightedList v w)
    -> w
    -> (WeightedItem v w)

get (WeightedList list) i
    = get' list i 0
  where
    get' :: [WeightedItem v w]
         -> w
         -> w
         -> (WeightedItem v w)

    get' [] _ _
      = error "Cannot access an empty WeightedList"
    
    get' (item:items) i t
        | t' > i     = item
        | otherwise  = get' items i t'
      where
        t' = t + (weight item)
