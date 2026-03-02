module Test.WeightedList.Merging where

import Test.Tasty
import Test.Tasty.HUnit

import Utils
import Test.Syntax

import WeightedList


test_merging :: TestTree
test_merging = testGroup "Merging"
  [ test_collection "merge" test_merge
  , test_collection "mergeWith" test_mergeWith
  , test_collection "mergeSelf" test_mergeSelf
  ]


test_merge :: [Assertion]
test_merge =
  []
-- TODO
test_mergeWith :: [Assertion]
test_mergeWith =
  [
    mergeWith __ __ === []
  , mergeWith wl __ === wl
  , mergeWith __ wl === wl

    -- merge 1
  , mergeWith wl (wlist [ (1, "sup") ])
              === wlist [ (3, "sup"), (3, "nova"), (5, "shard") ]

    -- merge 3
  , mergeWith wl wl === wlist [ (4, "sup"), (6, "nova"), (10, "shard") ]
  
    -- append 1
  , mergeWith wl (wlist [ (13, "cortex") ])
              === wlist [ (2, "sup"), (3, "nova"), (5, "shard"), (13, "cortex") ]
  
    -- append 2
  , mergeWith wl (wlist [ (13, "cortex"), (20, "origin") ])
        === wl ++ wlist [ (13, "cortex"), (20, "origin") ]
  
    -- append 3
  , mergeWith wl (wlist [ (13, "cortex"), (20, "origin"), (42, "vision") ])
        === wl ++ wlist [ (13, "cortex"), (20, "origin"), (42, "vision") ]
  ]

test_mergeSelf :: [Assertion]
test_mergeSelf =
  [
    mergeSelf __ === []

  , mergeSelf wl === wl

  , mergeSelf (wl ++ wl) === wlist [ (4, "sup"), (6, "nova"), (10, "shard") ]

  , mergeSelf (wlist [ (1, "sup"), (2, "sup"), (3, "sup") ])
           === wlist [ (6, "sup") ]
  ]
