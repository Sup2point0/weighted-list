module Test_WeightedList where

import Test.Tasty
import Test.Tasty.HUnit
import Test.Tasty.ExpectedFailure

import Data.Tuple

import Syntax
import WeightedList


---------------------------------------------------------------------

test_weighted_list :: TestTree
test_weighted_list = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  , test_collection "properties" test_properties
  , test_collection "indexing" test_indexing
  , test_collection "merging" test_merging
  ]

test_weighted_list_errors :: TestTree
test_weighted_list_errors = expectFail $ testGroup "WeightedList Errors"
  [ test_collection "indexing" test_indexing_errors
  ]


---------------------------------------------------------------------

__ = newWeightedList @String @Int []

l  = [(2, "sup"), (3, "nova"), (7, "shard")]

wl :: WeightedList String Int
wl = newWeightedList l


---------------------------------------------------------------------

test_constructor :: [Assertion]
test_constructor =
  [
    __ === []

  , wl === [ WeightedItem { value = "sup", weight = 2, c_weight = 0 }
           , WeightedItem { value = "nova", weight = 3, c_weight = 0 }
           , WeightedItem { value = "shard", weight = 7, c_weight = 0 }
           ]
  ]

test_properties :: [Assertion]
test_properties =
  [
    total_values wl === 3
  , total_weights wl === 12
  , values wl === ["sup", "nova", "shard"]
  , weights wl === [2, 3, 7]
  , raw wl === l
  , raw' wl === map swap l
  ]

test_indexing :: [Assertion]
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

  , value (get wl (-1))  === "shard"
  , value (get wl (-2))  === "shard"
  , value (get wl (-3))  === "shard"
  , value (get wl (-4))  === "shard"
  , value (get wl (-5))  === "shard"
  , value (get wl (-6))  === "shard"
  , value (get wl (-7))  === "shard"
  , value (get wl (-8))  === "nova"
  , value (get wl (-9))  === "nova"
  , value (get wl (-10)) === "nova"
  , value (get wl (-11)) === "sup"
  , value (get wl (-12)) === "sup"
  ]

test_indexing_errors :: [Assertion]
test_indexing_errors =
  [ Just (get wl (12)) === Nothing
  , Just (get wl (-13)) === Nothing
  ]

test_merging :: [Assertion]
test_merging =
  [
    merge __ __ === []
  , merge wl __ === wl
  , merge __ wl === wl

    -- merge 1
  , merge wl (newWeightedList [ (1, "sup") ])
          === newWeightedList [ (3, "sup"), (3, "nova"), (7, "shard") ]

    -- merge 3
  , merge wl wl === newWeightedList [ (4, "sup"), (6, "nova"), (14, "shard") ]
  
    -- append 1
  , merge wl (newWeightedList [ (13, "cortex") ])
          === newWeightedList [ (2, "sup"), (3, "nova"), (7, "shard"), (13, "cortex") ]
  
    -- append 2
  , merge wl (newWeightedList [ (13, "cortex"), (20, "origin") ])
          === wl ++ newWeightedList [ (13, "cortex"), (20, "origin") ]
  
    -- append 3
  , merge wl (newWeightedList [ (13, "cortex"), (20, "origin"), (42, "vision") ])
          === wl ++ newWeightedList [ (13, "cortex"), (20, "origin"), (42, "vision") ]
  ]
