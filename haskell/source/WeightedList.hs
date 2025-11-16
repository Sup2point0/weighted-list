{-|
weighted-list
by Sup#2.0 (@Sup2point0)
-}

module WeightedList where

import Data.List
import Data.Either


---------------------------------------------------------------------

type WeightedList v w = [WeightedItem v w]


data WeightedItem v w = WeightedItem
    { value :: v
    , weight :: w
    , c_weight :: w  -- cumulative weight for binary search
    }

instance (Show v, Show w) => Show (WeightedItem v w) where
  show (WeightedItem value weight _) = (
      "{ value = " ++ show value ++ ", weight = " ++ show weight ++ " }"
    )

instance (Eq v, Eq w) => Eq (WeightedItem v w) where
  item == item'
    = (
      value item == value item'
      && weight item == weight item'
    )

instance (Eq v, Eq w, Ord w) => Ord (WeightedItem v w) where
  item <= item' = weight item <= weight item'


{-| CONSTRUCTOR -}
---------------------------------------------------------------------

{-|
Construct a list of `WeightedItem`s from the provided (weight, value) pairs.
-}
newWeightedList :: forall v w. Num w
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


{-| ACCESSORS -}
---------------------------------------------------------------------

{- |
Sum the total weights of all items in a `WeightedList`.
-}
totalWeights :: (Num w) => WeightedList v w -> w
totalWeights = foldl' (\t item -> t + weight item) 0

totalWeights' :: (Num w) => WeightedList v w -> w
totalWeights' = sum . map weight

{- |
Count the total number of items in a `WeightedList`.
-}
totalValues :: WeightedList v w -> Int
totalValues = length

{- |
Get a list of the weights of all items in a `WeightedList`.
-}
weights :: WeightedList v w -> [w]
weights = map weight

{- |
Get a list of the values of all items in a `WeightedList`.
-}
values :: WeightedList v w -> [v]
values = map value

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


{-| SINGLE METHODS -}
---------------------------------------------------------------------

{- |
Get the item of a `WeightedList` at a weighted index.

If a negative index is provided, access starts from the end of the list (where the last item is at index `-1`).
 
Note that this has $O(n)$ time complexity.
-}
get :: forall v w. (Num w, Ord w)
    => WeightedList v w
    -> w
    -> WeightedItem v w

get [] _ = error "Cannot access an empty WeightedList"

get list i
    | i < 0     = fromRight (error "Failed to access WeightedList")
                            (foldr get_r (Left (fromIntegral 0)) list)
    | otherwise = get' list 0
  where
    get' :: WeightedList v w
         -> w
         -> WeightedItem v w
    get' [] _ = error "Index exceeded length of WeightedList"
    get' (item:items) t
        | t' > i    = item
        | otherwise = get' items t'
      where
        t' = t + weight item
    
    get_r :: WeightedItem v w
          -> Either w (WeightedItem v w)
          -> Either w (WeightedItem v w)
    get_r item (Right out) = Right out
    get_r item (Left acc)
        | t' >= (-i) = Right item
        | otherwise  = Left t'
      where
        t' = acc + weight item

{-|
Reduce the weight of the item at a given index by 1. If it becomes 0 as a result, remove the item from the list.
-}
pop :: forall v w. (Num w, Ord w)
    => WeightedList v w
    -> w
    -> WeightedList v w 

pop list i = popBy list i 1

{-|
Reduce the weight of the item at a given index by n. If it is no longer positive as a result, remove the item from the list.
-}
popBy :: forall v w. (Num w, Ord w)
      => WeightedList v w
      -> w
      -> w
      -> WeightedList v w

popBy [] _ _ = error "Cannot access an empty WeightedList"

popBy list i n
    = pop' list 0
  where
    pop' :: WeightedList v w
         -> w
         -> WeightedList v w
    pop' [] t = error "Index exceeded length of WeightedList"
    pop' (item:items) t
        | t' > i    = if weight item' > 0 then item' : items else items
        | otherwise = item : pop' items t'
      where
        t' = t + weight item
        item' = item { weight = weight item - n }


{-| SPECIAL METHODS -}
---------------------------------------------------------------------

{-|
Merge an item into the list. If an instance already exists, that instance’s weight is increased; otherwise, the item is appended to the end.
-}
merge :: (Eq v, Num w)
      => WeightedList v w
      -> WeightedItem v w
      -> WeightedList v w
merge [] item = [item]
merge (cand:rest) item
    | value cand == value item = cand' : rest
    | otherwise                = cand : merge rest item
  where
    cand' = cand { weight = weight cand + weight item }

{-|
Remove all items with non-positive weight.
-}
prune :: (Num w, Ord w)
      => WeightedList v w
      -> WeightedList v w
prune [] = []
prune (item:rest)
  | weight item > 0 = item : prune rest
  | otherwise       = prune rest

{-|
-}
collapse :: (Eq v, Num w)
         => WeightedList v w
         -> WeightedList v w
collapse list = mergeWith [] list

{-|
Merge 2 `WeightedList`s. Items from the right list are merged with items in the left list (if they share an equal value), otherwise they are appended in order.
-}
mergeWith :: (Eq v, Num w)
          => WeightedList v w
          -> WeightedList v w
          -> WeightedList v w

mergeWith []   list' = list'
mergeWith list []    = list
mergeWith list list' = foldl' merge list list'
