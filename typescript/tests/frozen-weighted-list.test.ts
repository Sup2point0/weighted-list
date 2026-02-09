import { test, assert } from "vitest";

import { FrozenWeightedList } from "../weighted-list/frozen-weighted-list";


const fwl = new FrozenWeightedList(
  [2, "sup"],
  [3, "nova"],
  [5, "shard"],
);


test("accessors", () =>
{
  assert.deepEqual( fwl.weights(), [2, 3, 5] );
  assert.deepEqual( fwl.values(), ["sup", "nova", "shard"] );

  assert.deepEqual( fwl.items(), [
    { weight: 2, value: "sup"   },
    { weight: 3, value: "nova"  },
    { weight: 5, value: "shard" },
  ]);
  
  assert.deepEqual( Array.from(fwl.entries()), [
    [0, { weight: 2, value: "sup"   }],
    [1, { weight: 3, value: "nova"  }],
    [2, { weight: 5, value: "shard" }],
  ]);

  assert.deepEqual( fwl.raw(), [
    [2, "sup"],
    [3, "nova"],
    [5, "shard"],
  ]);

  assert.deepEqual( Array.from(fwl.expanded()), [
    "sup", "sup",
    "nova", "nova", "nova",
    "shard", "shard", "shard", "shard", "shard"
  ])
})

test("properties", () =>
{
  assert.equal( fwl.length,        10 );
  assert.equal( fwl.total_weights, 10 );
  assert.equal( fwl.total_values,  3  );
});

test("at", () =>
{
  assert.equal( fwl.at(0)!.value, "sup"   );
  assert.equal( fwl.at(1)!.value, "sup"   );
  assert.equal( fwl.at(2)!.value, "nova"  );
  assert.equal( fwl.at(3)!.value, "nova"  );
  assert.equal( fwl.at(4)!.value, "nova"  );
  assert.equal( fwl.at(5)!.value, "shard" );
  assert.equal( fwl.at(6)!.value, "shard" );
  assert.equal( fwl.at(7)!.value, "shard" );
  assert.equal( fwl.at(8)!.value, "shard" );
  assert.equal( fwl.at(9)!.value, "shard" );
  assert.equal( fwl.at(12), undefined );
});
