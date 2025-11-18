module Test.WeightedList where

import Test.Tasty
import Test.Tasty.HUnit
import Test.Tasty.ExpectedFailure

import Data.List
import Data.Tuple

import Test.Syntax
import WeightedList


---------------------------------------------------------------------

test_weighted_list :: TestTree
test_weighted_list = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  , test_collection "properties" test_properties
  , test_collection "index" test_index
  , test_collection "pop" test_pop
  , test_collection "merge" test_merge
  , test_collection "typeclasses" test_typeclasses
  ]

test_weighted_list_errors :: TestTree
test_weighted_list_errors = expectFail $ testGroup "WeightedList Errors"
  [ test_collection "index" test_index_errors
  , test_collection "pop" test_pop_errors
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

  , wl === [ WeightedItem { value = "sup", weight = 2 }
           , WeightedItem { value = "nova", weight = 3 }
           , WeightedItem { value = "shard", weight = 7 }
           ]
  ]

test_properties :: [Assertion]
test_properties =
  [
    totalValues  __ === 0
  , totalValues  wl === 3
  , totalWeights __ === 0
  , totalWeights wl === 12

  , values  __ === []
  , values  wl === ["sup", "nova", "shard"]
  , weights __ === []
  , weights wl === [2, 3, 7]

  , raw  __ === []
  , raw  wl === l
  , raw' __ === []
  , raw' wl === map swap l
  ]

test_index :: [Assertion]
test_index =
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

test_index_errors :: [Assertion]
test_index_errors =
  [
    Just (get wl (12)) === Nothing
  , Just (get wl (-13)) === Nothing
  ]

test_pop :: [Assertion]
test_pop =
  [
    -- pop "sup"
    pop wl 0 === newWeightedList [ (1, "sup"), (3, "nova"), (7, "shard") ]
  , pop wl 1 === newWeightedList [ (1, "sup"), (3, "nova"), (7, "shard") ]

    -- pop "nova"
  , pop wl 2 === newWeightedList [ (2, "sup"), (2, "nova"), (7, "shard") ]
  , pop wl 3 === newWeightedList [ (2, "sup"), (2, "nova"), (7, "shard") ]
  , pop wl 4 === newWeightedList [ (2, "sup"), (2, "nova"), (7, "shard") ]

    -- pop "shard"
  , pop wl 5  === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , pop wl 6  === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , pop wl 7  === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , pop wl 8  === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , pop wl 9  === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , pop wl 10 === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , pop wl 11 === newWeightedList [ (2, "sup"), (3, "nova"), (6, "shard") ]

    -- pop entirely
  , popBy wl 0 2 === newWeightedList [             (3, "nova"), (7, "shard") ]
  , popBy wl 2 3 === newWeightedList [ (2, "sup"),              (7, "shard") ]
  , popBy wl 5 7 === newWeightedList [ (2, "sup"), (3, "nova")               ]

    -- pop negative
  , popBy wl 0 3 === newWeightedList [             (3, "nova"), (7, "shard") ]
  , popBy wl 2 4 === newWeightedList [ (2, "sup"),              (7, "shard") ]
  , popBy wl 5 8 === newWeightedList [ (2, "sup"), (3, "nova")               ]
  ]

test_pop_errors :: [Assertion]
test_pop_errors =
  [
    Just (pop __ 0) === Nothing
  , Just (pop wl 12) === Nothing
  , Just (popBy wl 12 1) === Nothing
  ]

test_prune :: [Assertion]
test_prune =
  [
    prune __ === __
  , prune wl === wl
  , prune (newWeightedList [ (0, "sup") ]) === __
  ]

test_collapse :: [Assertion]
test_collapse =
  [
    collapse __ === []
  , collapse wl === wl
  , collapse wl ++ wl === newWeightedList [ (4, "sup"), (6, "nova"), (14, "shard") ]
  , collapse (newWeightedList [ (1, "sup"), (2, "sup"), (3, "sup") ])
          === newWeightedList [ (3, "sup") ]
  ]

test_merge :: [Assertion]
test_merge =
  [
    mergeWith __ __ === []
  , mergeWith wl __ === wl
  , mergeWith __ wl === wl

    -- merge 1
  , mergeWith wl (newWeightedList [ (1, "sup") ])
              === newWeightedList [ (3, "sup"), (3, "nova"), (7, "shard") ]

    -- merge 3
  , mergeWith wl wl === newWeightedList [ (4, "sup"), (6, "nova"), (14, "shard") ]
  
    -- append 1
  , mergeWith wl (newWeightedList [ (13, "cortex") ])
              === newWeightedList [ (2, "sup"), (3, "nova"), (7, "shard"), (13, "cortex") ]
  
    -- append 2
  , mergeWith wl (newWeightedList [ (13, "cortex"), (20, "origin") ])
        === wl ++ newWeightedList [ (13, "cortex"), (20, "origin") ]
  
    -- append 3
  , mergeWith wl (newWeightedList [ (13, "cortex"), (20, "origin"), (42, "vision") ])
        === wl ++ newWeightedList [ (13, "cortex"), (20, "origin"), (42, "vision") ]
  ]

test_typeclasses :: [Assertion]
test_typeclasses =
  [
    sort wl === wl
  , sort (reverse wl) === wl
  ]
