module Test.WeightedList.Properties where

import Test.Tasty
import Test.Tasty.HUnit

import Utils
import Test.Syntax

import WeightedList


test_properties :: TestTree
test_properties = testGroup "Properties"
  [ test_collection "totalWeights" test_totalWeights
  , test_collection "totalItems" test_totalItems
  ]


test_totalWeights :: [Assertion]
test_totalWeights =
  [
    totalWeights __ === 0
  , totalWeights wl === 12
  ]

test_totalItems :: [Assertion]
test_totalItems =
  [
    totalItems __ === 0
  , totalItems wl === 3
  ]
