module Test.WeightedList.Random where

import Test.Tasty
import Test.Tasty.HUnit

import GHC.IO (unsafePerformIO)

import Utils
import Test.Syntax

import WeightedList


_TRIALS :: Int
_TRIALS = 42


test_random :: TestTree
test_random = testGroup "Random"
  [ test_collection "randomValue" test_randomValue
  , test_collection "randomValues" test_randomValues
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
