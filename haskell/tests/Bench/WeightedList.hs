module Bench.WeightedList where

import Test.Tasty.Bench
import Control.DeepSeq

import Utils

import WeightedList


bench_weighted_list :: [Benchmark]
bench_weighted_list =
  [ bgroup "properties" bench_properties
  , bgroup "indexing" bench_indexing
  ]

bench_properties :: [Benchmark]
bench_properties =
    [ bgroup "totalWeights"  bench_total_weights
    , bgroup "totalWeights'" bench_total_weights'
    ]
  where
    bench_total_weights :: [Benchmark]
    bench_total_weights =
      [ bench "wll 256 "  $ nf totalWeights (wll 256)
      , bench "wll 1024 " $ nf totalWeights (wll 1024)
      , bench "wll 4096 " $ nf totalWeights (wll 4096)
      ]

    bench_total_weights' :: [Benchmark]
    bench_total_weights' =
      [ bench "tll 256 "  $ nf (sum . map fst) (tll 256)
      , bench "tll 1024 " $ nf (sum . map fst) (tll 1024)
      , bench "tll 4096 " $ nf (sum . map fst) (tll 4096)
      ]

bench_indexing :: [Benchmark]
bench_indexing =
    [ bgroup "get"  bench_get
    , bgroup "get'" bench_get'
    ]
  where
    bench_get :: [Benchmark]
    bench_get =
      [ bench "wll 256 "  $ nf (value . get (wll 256))  255
      , bench "wll 1024 " $ nf (value . get (wll 1024)) 1023
      , bench "wll 4096 " $ nf (value . get (wll 4096)) 4095
      ]
      
    bench_get' :: [Benchmark]
    bench_get' =
      [ bench "tll 256 "  $ nf ([0..256  :: Int] !!)  255
      , bench "tll 1024 " $ nf ([0..1024 :: Int] !!) 1023
      , bench "tll 4096 " $ nf ([0..4096 :: Int] !!) 4095
      ]
