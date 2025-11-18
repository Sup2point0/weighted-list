module Bench.WeightedList where

import Test.Tasty.Bench
import Control.DeepSeq

import Utils

import WeightedList


bench_weighted_list :: [Benchmark]
bench_weighted_list =
  [ bgroup "properties" bench_properties
  ]

bench_properties :: [Benchmark]
bench_properties =
    [ bgroup "totalWeights" total_weights
    , bgroup "totalWeights'" total_weights'
    ]
  where
    total_weights :: [Benchmark]
    total_weights =
      [ bench "" $ nf totalWeights (wll 256)
      , bench "" $ nf totalWeights (wll 1024)
      , bench "" $ nf totalWeights (wll 4096)
      ]

    total_weights' :: [Benchmark]
    total_weights' =
      [ bench "" $ nf totalWeights' (wll 256)
      , bench "" $ nf totalWeights' (wll 1024)
      , bench "" $ nf totalWeights' (wll 4096)
      ]
