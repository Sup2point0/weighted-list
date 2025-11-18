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
      [ bench "totalWeights wll 256" $ nf totalWeights (wll 256)
      , bench "totalWeights wll 1024" $ nf totalWeights (wll 1024)
      , bench "totalWeights wll 4096" $ nf totalWeights (wll 4096)
      ]

    total_weights' :: [Benchmark]
    total_weights' =
      [ bench "totalWeights tll 256" $ nf (sum . map fst) (tll 256)
      , bench "totalWeights tll 1024" $ nf (sum . map fst) (tll 1024)
      , bench "totalWeights tll 4096" $ nf (sum . map fst) (tll 4096)
      ]
