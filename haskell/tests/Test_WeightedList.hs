module TestMatrix where

import Test.Tasty
import Test.Tasty.HUnit

import Syntax


test_weighted_list = testGroup "WeightedList"
  [ test_collection "constructor" test_constructor
  ] :: TestTree

test_constructor =
  [ newWeightedList [] === []
  ] :: [Assertion]
