module Test_WeightedList where

import Test.Tasty
import Test.Tasty.HUnit

import Data.Tuple

import Syntax
import WeightedList


test_weighted_list = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  , test_collection "properties" test_properties
  , test_collection "indexing" test_indexing
  ] :: TestTree


l  = [(2, "sup"), (3, "nova"), (7, "shard")]
wl = newWeightedList l
  :: WeightedList String Int


test_constructor =
  [
    newWeightedList @String @Int [] === []

  , wl === [ WeightedItem { value = "sup", weight = 2 }
           , WeightedItem { value = "nova", weight = 3 }
           , WeightedItem { value = "shard", weight = 7 }
           ]
  
  ] :: [Assertion]


test_properties =
  [
    total_values wl === 3
  , total_weights wl === 12
  , values wl === ["sup", "nova", "shard"]
  , weights wl === [2, 3, 7]
  , raw wl === l
  , raw' wl === map swap l
  ]


test_indexing =
  [
    value (get wl 0)  === "sup"
  , value (get wl 1)  === "sup"
  , value (get wl 2)  === "nova"
  , value (get wl 3)  === "nova"
  , value (get wl 4)  === "nova"
  , value (get wl 5)  === "shard"
  , value (get wl 6)  === "shard"
  , value (get wl 7)  === "shard"
  , value (get wl 8)  === "shard"
  , value (get wl 9)  === "shard"
  , value (get wl 10) === "shard"
  , value (get wl 11) === "shard"
  
  ] :: [Assertion]
