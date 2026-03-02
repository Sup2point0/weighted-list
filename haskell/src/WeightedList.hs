{-|
weighted-list
by Sup#2.0 (@Sup2point0)
-}

module WeightedList where

import Data.Bifunctor
import Data.Either
import Data.List
import Data.Tuple

import System.Random


---------------------------------------------------------------------

type WeightedList v w = [Item w v]


data Item w v = Item
  { weight :: w
  , value :: v
  }

instance (Show v, Show w) => Show (Item w v) where
  show (Item weight value)
    = "{ weight = " ++ show weight ++ ", value = " ++ show value ++ " }"

instance (Eq v, Eq w) => Eq (Item w v) where
  item == item' = value item == value item'
                && weight item == weight item'

instance (Eq v, Eq w, Ord w) => Ord (Item w v) where
  item <= item' = weight item <= weight item'

instance Functor (Item w) where
  fmap f (Item weight value) = Item weight (f value)

instance Bifunctor Item where
  bimap f g (Item weight value) = Item (f weight) (g value)


-------------------- CONSTRUCTORS --------------------

{-|
Construct a list of `Item`s from the provided (weight, value) pairs.
-}
newWeightedList :: forall v w. (Num w)  -- NOTE: `forall` to enforce `v, w` order
                => [(w, v)]
                -> WeightedList v w
newWeightedList []    = []
newWeightedList items = map (uncurry Item) items

wlist :: forall v w. (Num w) => [(w, v)] -> WeightedList v w
wlist = newWeightedList


-------------------- PROPERTIES --------------------

{-| Sum the total weights of all items in a `WeightedList`.
-}
totalWeights :: (Num w) => WeightedList v w -> w
totalWeights list = foldl' (\t item -> t + weight item) 0 list

{-| Count the total number of items in a `WeightedList`.
-}
totalItems :: WeightedList v w -> Int
totalItems list = length list

{-| Do all items (if any) have a weight of zero?
-}
isZero :: (Num w, Eq w) => WeightedList v w -> Bool
isZero list = all (\item -> weight item == 0) list

{-| Do any items have a negative weight?
-}
hasNegativeWeights :: (Num w, Ord w) => WeightedList v w -> Bool
hasNegativeWeights list = any (\item -> weight item < 0) list


-------------------- ACCESSORS --------------------

{-| Get a list of the weights of all items in a `WeightedList`.
-}
weights :: WeightedList v w -> [w]
weights = map weight

{-| Get a list of the values of all items in a `WeightedList`.
-}
values :: WeightedList v w -> [v]
values = map value

{-| Get the raw representation of a `WeightedList` in (weight, value) pairs.

This satisfies the axiom `newWeightedList (raw (newWeightedList list)) == newWeightedList list`.
-}
raw :: WeightedList v w -> [(w, v)]
raw = map (\item -> (weight item, value item))

{-| Get the raw representation of a `WeightedList` in (value, weight) pairs.
-}
raw' :: WeightedList v w -> [(v, w)]
raw' = map (\item -> (value item, weight item))

expanded :: WeightedList v Int -> [v]
expanded []           = []
expanded (item:items) = replicate (weight item) (value item) ++ expanded items


-------------------- LIST METHODS --------------------

{-| Get the item of a `WeightedList` at a weighted index.

If a negative index is provided, access starts from the end of the list (where the last item is at index `-1`).
 
Note that this has $O(n)$ time complexity.
-}
get :: forall v w. (Num w, Ord w)
    => WeightedList v w
    -> w
    -> Item w v

get [] _ = error "Cannot access an empty WeightedList"

get list i
  | i < 0     = fromRight (error "Failed to access WeightedList") (foldr get_r (Left 0) list)
  | otherwise = get' list 0
  where
    get' :: WeightedList v w
         -> w
         -> Item w v
    get' [] _ = error "Index exceeded length of WeightedList"
    get' (item:items) t
      | t' > i    = item
      | otherwise = get' items t'
      where
        t' = t + weight item
    
    get_r :: Item w v
          -> Either w (Item w v)
          -> Either w (Item w v)
    get_r item (Right out) = Right out
    get_r item (Left acc)
        | t' >= (-i) = Right item
        | otherwise  = Left t'
      where
        t' = acc + weight item

{-| Try to get the item of a `WeightedList` at a weighted index, returning `Nothing` if the index is out of bounds.

If a negative index is provided, access starts from the end of the list (where the last item is at index `-1`).
 
Note that this has $O(n)$ time complexity.
-}
tryGet :: forall v w. (Num w, Ord w)
       => WeightedList v w
       -> w
       -> Maybe (Item w v)

tryGet [] _ = Nothing

tryGet list i
  | i < 0     = either (const Nothing) Just $ foldr get_r (Left 0) list
  | otherwise = get' list 0
  where
    get' :: (WeightedList v w) -> w -> Maybe (Item w v)
    get' [] _ = Nothing
    get' (item:items) t
      | t' > i    = Just item
      | otherwise = get' items t'
      where
        t' = t + weight item
    
    get_r :: Item w v
          -> Either w (Item w v)
          -> Either w (Item w v)
    get_r item (Right out) = Right out
    get_r item (Left acc)
        | t' >= (-i) = Right item
        | otherwise  = Left t'
      where
        t' = acc + weight item

{-|
Reduce the weight of the item at a given index by 1. If it becomes 0 as a result, remove the item from the list.
-}
takeAt :: forall v w. (Num w, Ord w)
       => WeightedList v w
       -> w
       -> WeightedList v w 

takeAt list i = takeByAt list 1 i

{-|
Reduce the weight of the item at a given index by n. If it is no longer positive as a result, remove the item from the list.
-}
takeByAt :: forall v w. (Num w, Ord w)
         => WeightedList v w
         -> w
         -> w
         -> WeightedList v w

takeByAt [] _ _ = error "Cannot access an empty WeightedList"

takeByAt list n i
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


-------------------- WEIGHTEDLIST METHODS --------------------

{-| Merge an item into the list. If an instance already exists, that instance’s weight is increased; otherwise, the item is appended to the end.
-}
merge :: (Eq v, Num w)
      => WeightedList v w
      -> Item w v
      -> WeightedList v w
merge [] item = [item]
merge (cand:rest) item
  | value cand == value item = cand' : rest
  | otherwise                = cand : merge rest item
  where
    cand' = cand { weight = weight cand + weight item }

{-| Merge 2 `WeightedList`s. Items from the right list are merged with items in the left list (if they share an equal value), otherwise they are appended in order.
-}
mergeWith :: (Eq v, Num w)
          => WeightedList v w
          -> WeightedList v w
          -> WeightedList v w
mergeWith list []    = list
mergeWith list list' = foldl' merge list list'

{-|
-}
mergeSelf :: (Eq v, Num w)
          => WeightedList v w
          -> WeightedList v w
mergeSelf list = mergeWith [] list

{-| Remove all items with non-positive weight.
-}
prune :: (Num w, Ord w)
      => WeightedList v w
      -> WeightedList v w
prune [] = []
prune (item:rest)
  | weight item > 0 = item : prune rest
  | otherwise       = prune rest


-------------------- SAMPLING METHODS --------------------

_randomIndexTo :: Int -> IO Int
_randomIndexTo n = do
  i <- randomIO :: IO Float
  return $ floor (i * fromIntegral n)

randomItem :: WeightedList v Int -> IO (Item Int v)
randomItem list = do
  i <- _randomIndexTo (totalWeights list)
  return (get list i)

randomValue :: WeightedList v Int -> IO v
randomValue = fmap value . randomItem

randomItemTo :: Int -> WeightedList v Int -> IO (Item Int v)
randomItemTo n list = do
  i <- _randomIndexTo n
  return (get list i)

randomValueTo :: Int -> WeightedList v Int -> IO v
randomValueTo n list = fmap value (randomItemTo n list)

randomValues :: Int -> WeightedList v Int -> IO [v]
randomValues n list = go n list
  where
    !l = length list
    go 0 list = return []
    go n list = (:) <$> randomValueTo l list <*> go (n-1) list
