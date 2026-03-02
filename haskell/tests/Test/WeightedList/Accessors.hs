module Test.WeightedList.Accessors where

import Test.Tasty
import Test.Tasty.HUnit
  
import Data.Tuple

import Utils
import Test.Syntax

import WeightedList


test_accessors :: TestTree
test_accessors = testGroup "Accessors"
  [ test_collection "weights" test_weights
  , test_collection "values" test_values
  , test_collection "raw" test_raw
  ]


test_weights :: [Assertion]
test_weights =
  [
    weights __ === []
  , weights wl === [2, 3, 5]
  ]

test_values :: [Assertion]
test_values =
  [
    values  __ === []
  , values  wl === ["sup", "nova", "shard"]
  ]

test_raw :: [Assertion]
test_raw =
  [
    raw  __ === []
  , raw  wl === tl

  , raw' __ === []
  , raw' wl === map swap tl
  ]
