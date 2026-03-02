module Test.WeightedList.Typeclasses where

import Test.Tasty
import Test.Tasty.HUnit

import Data.List
import Data.Text qualified as Text

import Utils
import Test.Syntax

import WeightedList


test_typeclasses :: TestTree
test_typeclasses = testGroup "Typeclasses"
  [ test_collection "Functor" test_functor
  , test_collection "Ord" test_ord
  ]


test_functor :: [Assertion]
test_functor =
  [
    map (fmap (Text.unpack . Text.toUpper . Text.pack)) wl
      === wlist [(2, "SUP"), (3, "NOVA"), (5, "SHARD")]
  ]

test_ord :: [Assertion]
test_ord =
  [
    sort wl === wl
  , sort (reverse wl) === wl
  ]
