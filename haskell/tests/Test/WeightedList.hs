module Test.WeightedList where

import Test.Tasty
import Test.Tasty.HUnit
import Test.Tasty.ExpectedFailure

import Data.List
import Data.Text qualified as Text
import Data.Tuple

import Utils
import Test.Syntax

import WeightedList


---------------------------------------------------------------------

test_weighted_list :: TestTree
test_weighted_list = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  , test_collection "properties" test_properties
  , test_collection "index" test_index
  , test_collection "take" test_take
  , test_collection "mergeWith" test_mergeWith
  , test_collection "typeclasses" test_typeclasses
  ]

test_weighted_list_errors :: TestTree
test_weighted_list_errors = expectFail $ testGroup "WeightedList Errors"
  [ test_collection "index" test_index_errors
  , test_collection "take" test_take_errors
  ]


---------------------------------------------------------------------

test_constructor :: [Assertion]
test_constructor =
  [
    __ === []

  , wl === [ Item { value = "sup", weight = 2 }
           , Item { value = "nova", weight = 3 }
           , Item { value = "shard", weight = 7 }
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
  , raw  wl === tl
  , raw' __ === []
  , raw' wl === map swap tl
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
    Just (get wl   12 ) === Nothing
  , Just (get wl (-13)) === Nothing
  ]

test_take :: [Assertion]
test_take =
  [
    -- take "sup"
    takeAt wl 0 === wlist [ (1, "sup"), (3, "nova"), (7, "shard") ]
  , takeAt wl 1 === wlist [ (1, "sup"), (3, "nova"), (7, "shard") ]

    -- take "nova"
  , takeAt wl 2 === wlist [ (2, "sup"), (2, "nova"), (7, "shard") ]
  , takeAt wl 3 === wlist [ (2, "sup"), (2, "nova"), (7, "shard") ]
  , takeAt wl 4 === wlist [ (2, "sup"), (2, "nova"), (7, "shard") ]

    -- take "shard"
  , takeAt wl 5  === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , takeAt wl 6  === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , takeAt wl 7  === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , takeAt wl 8  === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , takeAt wl 9  === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , takeAt wl 10 === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]
  , takeAt wl 11 === wlist [ (2, "sup"), (3, "nova"), (6, "shard") ]

    -- take entirely
  , takeByAt wl 2 0 === wlist [             (3, "nova"), (7, "shard") ]
  , takeByAt wl 3 2 === wlist [ (2, "sup"),              (7, "shard") ]
  , takeByAt wl 7 5 === wlist [ (2, "sup"), (3, "nova")               ]

    -- take negative
  , takeByAt wl 3 0 === wlist [             (3, "nova"), (7, "shard") ]
  , takeByAt wl 4 2 === wlist [ (2, "sup"),              (7, "shard") ]
  , takeByAt wl 8 5 === wlist [ (2, "sup"), (3, "nova")               ]
  ]

test_take_errors :: [Assertion]
test_take_errors =
  [
    Just (takeAt __ 0) === Nothing
  , Just (takeAt wl 12) === Nothing
  , Just (takeByAt wl 1 12) === Nothing
  ]

test_mergeWith :: [Assertion]
test_mergeWith =
  [
    mergeWith __ __ === []
  , mergeWith wl __ === wl
  , mergeWith __ wl === wl

    -- merge 1
  , mergeWith wl (wlist [ (1, "sup") ])
              === wlist [ (3, "sup"), (3, "nova"), (7, "shard") ]

    -- merge 3
  , mergeWith wl wl === wlist [ (4, "sup"), (6, "nova"), (14, "shard") ]
  
    -- append 1
  , mergeWith wl (wlist [ (13, "cortex") ])
              === wlist [ (2, "sup"), (3, "nova"), (7, "shard"), (13, "cortex") ]
  
    -- append 2
  , mergeWith wl (wlist [ (13, "cortex"), (20, "origin") ])
        === wl ++ wlist [ (13, "cortex"), (20, "origin") ]
  
    -- append 3
  , mergeWith wl (wlist [ (13, "cortex"), (20, "origin"), (42, "vision") ])
        === wl ++ wlist [ (13, "cortex"), (20, "origin"), (42, "vision") ]
  ]

test_mergeDuplicates :: [Assertion]
test_mergeDuplicates =
  [
    mergeDuplicates __ === []
  , mergeDuplicates wl === wl
  , mergeDuplicates wl ++ wl === wlist [ (4, "sup"), (6, "nova"), (14, "shard") ]
  , mergeDuplicates (wlist [ (1, "sup"), (2, "sup"), (3, "sup") ])
          === wlist [ (3, "sup") ]
  ]

test_prune :: [Assertion]
test_prune =
  [
    prune __ === __
  , prune wl === wl
  , prune (wlist [ (0, "sup") ]) === __
  ]

test_typeclasses :: [Assertion]
test_typeclasses =
  [
    sort wl === wl
  , sort (reverse wl) === wl

  , map (fmap (Text.unpack . Text.toUpper . Text.pack)) wl === wlist [(2, "SUP"), (3, "NOVA"), (7, "SHARD")]
  ]
