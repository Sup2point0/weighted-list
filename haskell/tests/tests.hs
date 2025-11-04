import Test.Tasty

import Test_WeightedList


main :: IO ()
main = defaultMain tests


tests :: TestTree
tests = testGroup "weighted-list"
  [ test_weighted_list
  ]
