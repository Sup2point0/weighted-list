module Test_WeightedList where

import Test.Tasty
import Test.Tasty.HUnit

import WeightedList
import Syntax


test_weighted_list = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  ] :: TestTree


t = newWeightedList
  [(2, "sup"), (3, "nova"), (7, "shard")]
  :: WeightedList String Int


test_constructor =
  [ t ===
    [ WeightedItem { value = "sup", weight = 2 }
    , WeightedItem { value = "nova", weight = 3 }
    , WeightedItem { value = "shard", weight = 7 }
    ]
  ] :: [Assertion]
