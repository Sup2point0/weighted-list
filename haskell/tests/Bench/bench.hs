import Test.Tasty.Bench

import Bench.WeightedList


main :: IO ()
main = defaultMain benchmarks


benchmarks :: [Benchmark]
benchmarks = 
  [ bgroup "WeightedList" $ bench_weighted_list
  ]
