module Test.WeightedList.Taking where

import Test.Tasty
import Test.Tasty.HUnit

import Utils
import Test.Syntax

import WeightedList


test_taking :: TestTree
test_taking = testGroup "Taking"
  [ test_collection "takeAt" test_takeAt
  , test_collection "takeByAt" test_takeByAt
  ]

test_take_errors :: TestTree
test_take_errors = test_collection "Taking Errors"
  [
    Just (takeAt __ 0) === Nothing
  , Just (takeAt wl 12) === Nothing
  , Just (takeByAt wl 1 12) === Nothing
  ]


test_takeAt :: [Assertion]
test_takeAt =
  [
    -- take "sup"
    takeAt wl 0 === wlist [ (1, "sup"), (3, "nova"), (5, "shard") ]
  , takeAt wl 1 === wlist [ (1, "sup"), (3, "nova"), (5, "shard") ]

    -- take "nova"
  , takeAt wl 2 === wlist [ (2, "sup"), (2, "nova"), (5, "shard") ]
  , takeAt wl 3 === wlist [ (2, "sup"), (2, "nova"), (5, "shard") ]
  , takeAt wl 4 === wlist [ (2, "sup"), (2, "nova"), (5, "shard") ]

    -- take "shard"
  , takeAt wl 5  === wlist [ (2, "sup"), (3, "nova"), (4, "shard") ]
  , takeAt wl 6  === wlist [ (2, "sup"), (3, "nova"), (4, "shard") ]
  , takeAt wl 7  === wlist [ (2, "sup"), (3, "nova"), (4, "shard") ]
  , takeAt wl 8  === wlist [ (2, "sup"), (3, "nova"), (4, "shard") ]
  , takeAt wl 9  === wlist [ (2, "sup"), (3, "nova"), (4, "shard") ]
  ]

test_takeByAt :: [Assertion]
test_takeByAt =
  [
    -- take entirely
    takeByAt wl 2 0 === wlist [             (3, "nova"), (5, "shard") ]
  , takeByAt wl 3 2 === wlist [ (2, "sup"),              (5, "shard") ]
  , takeByAt wl 5 5 === wlist [ (2, "sup"), (3, "nova")               ]

    -- take negative
  , takeByAt wl 3 0 === wlist [             (3, "nova"), (5, "shard") ]
  , takeByAt wl 4 2 === wlist [ (2, "sup"),              (5, "shard") ]
  , takeByAt wl 6 5 === wlist [ (2, "sup"), (3, "nova")               ]
  ]
