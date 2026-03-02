import Test.Tasty

import Test.WeightedList


main :: IO ()
main = defaultMain tests


tests :: TestTree
tests = testGroup "weighted-list"
  [ test_WeightedList
  , test_WeightedList_errors
  ]
