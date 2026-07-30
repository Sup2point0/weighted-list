import { describe, test, assert } from "vitest";

import { FWList } from "../../src";
import { fwl } from "./shared";


describe("constructor", () =>
{
  test("from WeightedItems", () => {
    assert.deepEqual( fwl(), new FWList(
      { weight: 2, value: "sup"   },
      { weight: 3, value: "nova"  },
      { weight: 5, value: "shard" },
    ));
  });

  test("from FrozenWeightedItems", () => {
    assert.deepEqual( fwl(), new FWList(
      { weight: 2, value: "sup",   cumulative_weight: 0 },
      { weight: 3, value: "nova",  cumulative_weight: 0 },
      { weight: 5, value: "shard", cumulative_weight: 0 },
    ));
  });

  test("incorrect order", () => {
    assert.throws(() =>
      // if type-check error, good - weight should come first!
      new FWList(["sup", 2])
    );
  });

  test("negative weight", () => {
    assert.throws(() =>
      new FWList([-1, "eto"])
    );
  });

  test("NaN weight", () => {
    assert.throws(() =>
      new FWList([NaN, "nan"])
    );
  });

  test("infinite weight", () => {
    assert.throws(() =>
      new FWList([Infinity, "nan"])
    );
  });
});

