module Test.WeightedList.WeightManipulation where

import Test.Tasty
import Test.Tasty.HUnit

import Utils
import Test.Syntax

import WeightedList


test_weight_manipulation :: TestTree
test_weight_manipulation = testGroup "Weights"
  [ test_collection "prune" test_prune
  ]


test_prune :: [Assertion]
test_prune =
  [
    prune __ === __
  , prune wl === wl
  , prune (wlist [ (0, "sup") ]) === __
  ]
