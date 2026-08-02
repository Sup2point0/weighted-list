import { assert } from "vitest";
import quantile from "@stdlib/stats-base-dists-binomial-quantile";

import { WeightedCollection } from "../src";
import type { int } from "../src/shared";


const TRIALS = 20_000;
const CONFIDENCE_PERCENT = 99;
const CRITICAL_PERCENT = 100 - CONFIDENCE_PERCENT;
const SIGNIFICANCE_LEVEL = CRITICAL_PERCENT / 100 / 2;


export function test_binomial_for<Value>(
  method: "sample-value" | "sample-values" | "sample-values-unique",
  list: WeightedCollection<Value>,
): void
{
  const total = list.total_weight;

  for (let item of list.items()) {
    let prob = item.weight / total;

    let observed = 0;

    switch (method) {
      case "sample-value":
        for (let i = 0; i < TRIALS; i++) {
          if (list.sample_value() === item.value) observed++;
        }
        break;

      case "sample-values":
        observed = [...list.sample_values(TRIALS)].filter(val => val === item.value).length;
        break;
    }

    let lower_bound = quantile(SIGNIFICANCE_LEVEL, TRIALS, prob);
    let upper_bound = quantile(1 - SIGNIFICANCE_LEVEL, TRIALS, prob);

    assert.isAtLeast( observed, lower_bound );
    assert.isAtMost( observed, upper_bound );
  }
}
