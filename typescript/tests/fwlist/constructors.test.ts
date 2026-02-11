import { test, assert } from "vitest";

import { fwl, FWList } from "./shared";


test("constructors", () =>
{
  assert.deepEqual( fwl, new FWList(
    { weight: 2, value: "sup"   },
    { weight: 3, value: "nova"  },
    { weight: 5, value: "shard" },
  ));
  
  assert.deepEqual( fwl, new FWList(
    { weight: 2, value: "sup",   cumulative_weight: 0 },
    { weight: 3, value: "nova",  cumulative_weight: 0 },
    { weight: 5, value: "shard", cumulative_weight: 0 },
  ));

  assert.throws(() => new FWList(
    // if IDE is giving an error, good - weight should come first!
    ["sup", 2]
  ));
});

