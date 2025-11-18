import Test.Tasty

import Test.WeightedList


main :: IO ()
main = defaultMain tests


tests :: TestTree
tests = testGroup "weighted-list"
  [ test_weighted_list
  , test_weighted_list_errors
  ]
