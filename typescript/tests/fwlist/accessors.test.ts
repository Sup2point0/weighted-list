import { test, assert } from "vitest";

import { el, fwl, ofwl } from "./shared";


test("weights", () =>
{
  assert.deepEqual( el().weights(),  [] );
  assert.deepEqual( fwl().weights(), [2, 3, 5] );

  assert.deepEqual( Array.from(el().iter_weights()),  [] );
  assert.deepEqual( Array.from(fwl().iter_weights()), fwl().weights() );
});

test("values", () =>
{
  assert.deepEqual( el().values(),  [] );
  assert.deepEqual( fwl().values(), ["sup", "nova", "shard"] );

  assert.deepEqual( Array.from(el().iter_values()),  [] );
  assert.deepEqual( Array.from(fwl().iter_values()), fwl().values() );

  let item = ofwl().values()[0];
  assert.throws(() => { item.good = false; });
});

test("items", () =>
{
  assert.deepEqual( el().items(), [] );
  assert.deepEqual( fwl().items(), [
    { weight: 2, value: "sup"   },
    { weight: 3, value: "nova"  },
    { weight: 5, value: "shard" },
  ]);

  assert.deepEqual( Array.from(fwl().iter_items()), fwl().items() );

  let item = ofwl().items()[0];
  assert.throws(() => { item.value.good = false; });
});

test("entries", () =>
{
  assert.deepEqual( el().entries(), [] );
  assert.deepEqual( fwl().entries(), [
    [0, { weight: 2, value: "sup"   }],
    [1, { weight: 3, value: "nova"  }],
    [2, { weight: 5, value: "shard" }],
  ]);
  
  assert.deepEqual( Array.from(fwl().iter_entries()), fwl().entries());

  let item = ofwl().entries()[0];
  assert.throws(() => { item[1].value.good = false; });
});

test("raw", () =>
{
  assert.deepEqual( el().raw(), [] );
  assert.deepEqual( fwl().raw(), [
    [2, "sup"],
    [3, "nova"],
    [5, "shard"],
  ]);

  assert.deepEqual( Array.from(fwl().iter_raw()), fwl().raw() );

  let item = ofwl().raw()[0];
  assert.throws(() => { item[1].good = false; });
});

test("expanded", () =>
{
  assert.deepEqual( el().expanded(), [] );
  assert.deepEqual( fwl().expanded(), [
    "sup", "sup",
    "nova", "nova", "nova",
    "shard", "shard", "shard", "shard", "shard"
  ]);

  assert.deepEqual( Array.from(fwl().iter_expanded()), fwl().expanded() );

  let item = ofwl().expanded()[0];
  assert.throws(() => { item.good = false; });
})
