module Test.WeightedList where

import Test.Tasty
import Test.Tasty.HUnit
import Test.Tasty.ExpectedFailure

import Data.List
import Data.Text qualified as Text
import Data.Tuple

import GHC.IO (unsafePerformIO)

import Utils
import Test.Syntax

import WeightedList


import Test.WeightedList.Properties
import Test.WeightedList.Accessors
import Test.WeightedList.Indexing


---------------------------------------------------------------------

_TRIALS :: Int
_TRIALS = 42


---------------------------------------------------------------------

test_WeightedList :: TestTree
test_WeightedList = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  , test_properties
  , test_accessors
  , test_indexing
  , test_collection "take" test_take
  , test_collection "mergeWith" test_mergeWith
  , test_collection "randomValue" test_randomValue
  , test_collection "randomValues" test_randomValues
  , test_collection "typeclasses" test_typeclasses
  ]

test_WeightedList_errors :: TestTree
test_WeightedList_errors = expectFail $ testGroup "WeightedList Errors"
  [ test_collection "index-out-of-bounds" test_indexing_errors
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

test_randomValue :: [Assertion]
test_randomValue =
  [
    unsafePerformIO (randomValue wl) `elem` ["sup", "nova", "shard"] === True
  ]

test_randomValues :: [Assertion]
test_randomValues =
  [
    all (`elem` ["sup", "nova", "shard"]) (unsafePerformIO (randomValues _TRIALS wl)) === True
  ]

test_typeclasses :: [Assertion]
test_typeclasses =
  [
    sort wl === wl
  , sort (reverse wl) === wl

  , map (fmap (Text.unpack . Text.toUpper . Text.pack)) wl === wlist [(2, "SUP"), (3, "NOVA"), (7, "SHARD")]
  ]
