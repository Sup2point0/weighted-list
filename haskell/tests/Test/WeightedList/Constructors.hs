module Test.WeightedList.Constructors where

import Test.Tasty
import Test.Tasty.HUnit

import Utils
import Test.Syntax

import WeightedList


test_constructors :: TestTree
test_constructors = testGroup "Constructors"
  [ test_collection "[Item]" test_raw
  ]


test_raw :: [Assertion]
test_raw =
  [
    __ === []

  , wl === [ Item { value = "sup", weight = 2 }
           , Item { value = "nova", weight = 3 }
           , Item { value = "shard", weight = 5 }
           ]
  ]
