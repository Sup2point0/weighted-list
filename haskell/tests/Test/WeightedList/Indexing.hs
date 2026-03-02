module Test.WeightedList.Indexing where

import Test.Tasty
import Test.Tasty.HUnit

import Utils
import Test.Syntax

import WeightedList


test_indexing :: TestTree
test_indexing = testGroup "Indexing"
  [ test_collection "index" test_index
  , test_collection "index-negative" test_negative
  ]

test_indexing_errors :: [Assertion]
test_indexing_errors =
  [
    Just (get wl   12 ) === Nothing
  , Just (get wl (-13)) === Nothing
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
  ]

test_negative :: [Assertion]
test_negative =
  [
    value (get wl (-1))  === "shard"
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

