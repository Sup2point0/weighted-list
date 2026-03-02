module Test.WeightedList where

import Test.Tasty
import Test.Tasty.ExpectedFailure

import Test.WeightedList.Constructors
import Test.WeightedList.Properties
import Test.WeightedList.Accessors
import Test.WeightedList.Indexing
import Test.WeightedList.Taking
import Test.WeightedList.Merging
import Test.WeightedList.Random
import Test.WeightedList.WeightManipulation
import Test.WeightedList.Typeclasses


test_WeightedList :: TestTree
test_WeightedList = testGroup "WeightedList"
  [ test_constructors
  , test_properties
  , test_accessors
  , test_indexing
  , test_taking
  , test_merging
  , test_random
  , test_weight_manipulation
  , test_typeclasses
  ]

test_WeightedList_errors :: TestTree
test_WeightedList_errors = expectFail $ testGroup "WeightedList Errors"
  [ test_indexing_errors
  , test_take_errors
  ]
