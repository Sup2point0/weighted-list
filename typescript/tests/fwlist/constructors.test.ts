import { test, assert } from "vitest";

import { FWList } from "../../weighted-list";
import { fwl } from "./shared";


test("from-WeightedItems", () =>
{
  assert.deepEqual( fwl(), new FWList(
    { weight: 2, value: "sup"   },
    { weight: 3, value: "nova"  },
    { weight: 5, value: "shard" },
  ));
});

test("from-FrozenWeightedItems", () =>
{
  assert.deepEqual( fwl(), new FWList(
    { weight: 2, value: "sup",   cumulative_weight: 0 },
    { weight: 3, value: "nova",  cumulative_weight: 0 },
    { weight: 5, value: "shard", cumulative_weight: 0 },
  ));
});

test("incorrect-order", () =>
{
  assert.throws(() => new FWList(
    // if IDE is giving an error, good - weight should come first!
    ["sup", 2]
  ));
});

test("zero-weight", () =>
{
  assert.throws(() => new FWList(
    [0, "aleph"]
  ));
});

test("negative-weight", () =>
{
  assert.throws(() => new FWList(
    [-1, "eto"]
  ));
});

