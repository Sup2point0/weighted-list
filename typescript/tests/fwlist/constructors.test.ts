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
    assert.throws(() => new FWList(
      // if type-check error, good - weight should come first!
      ["sup", 2]
    ));
  });

  test("negative weight", () => {
    assert.throws(() => new FWList(
      [-1, "eto"]
    ));
  });

  test("NaN weight", () => {
    assert.throws(() => new FWList(
      [NaN, "nan"]
    ));
  });

  test("infinite weight", () => {
    assert.throws(() => new FWList(
      [Infinity, "nan"]
    ));
  });

  // if type-check errors, good!
  test("weird formats", () => {
    assert.throws(() => new FWList([1, 2, 3]));
    assert.throws(() => new FWList(["weight", "value"]));
  });
});

describe("from", () =>
{
  test("from WeightedItems", () => {
    assert.deepEqual( fwl(), FWList.from([
      { weight: 2, value: "sup"   },
      { weight: 3, value: "nova"  },
      { weight: 5, value: "shard" },
    ]));
  });

  test("from FrozenWeightedItems", () => {
    assert.deepEqual( fwl(), FWList.from([
      { weight: 2, value: "sup",   cumulative_weight: 0 },
      { weight: 3, value: "nova",  cumulative_weight: 0 },
      { weight: 5, value: "shard", cumulative_weight: 0 },
    ]));
  });

  test("incorrect order", () => {
    assert.throws(() =>
      // if type-check error, good - weight should come first!
      FWList.from([["sup", 2]])
    );
  });

  test("negative weight", () => {
    assert.throws(() =>
      FWList.from([[-1, "eto"]])
    );
  });

  test("NaN weight", () => {
    assert.throws(() =>
      FWList.from([[NaN, "nan"]])
    );
  });

  test("infinite weight", () => {
    assert.throws(() =>
      FWList.from([[Infinity, "nan"]])
    );
  });
});
