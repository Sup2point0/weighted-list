import { test, assert } from "vitest";

import { WeightedList } from "../weighted-list/weighted-list";


const wl = new WeightedList(
  [2, "sup"],
  [3, "nova"],
  [7, "shard"],
);


test("constructors", () => {
  let test = WeightedList.from_object({
    sup: 2,
    nova: 3,
    shard: 7,
  });

  assert.deepEqual( test, wl );
});

test("properties", () => {
  assert( wl.length        === 12 );
  assert( wl.total_weights === 12 );
  assert( wl.total_values  === 3 );

  assert.deepEqual( wl.weights(), [2, 3, 7] );
  assert.deepEqual( wl.values(), ["sup", "nova", "shard"] );
  
  assert.deepEqual( wl.entries().toArray(), [
    [0, { weight: 2, value: "sup" }],
    [1, { weight: 3, value: "nova" }],
    [2, { weight: 7, value: "shard" }],
  ]);
  assert.deepEqual( wl.raw(), [
    [2, "sup"],
    [3, "nova"],
    [7, "shard"],
  ]);
});

test("clone", () => {
  let t = wl.clone().clear();
  assert(t !== wl);
});

test("at", () => {
  assert( wl.at(0)!.value === "sup" );
  assert( wl.at(1)!.value === "sup" );
  assert( wl.at(2)!.value === "nova" );
  assert( wl.at(3)!.value === "nova" );
  assert( wl.at(4)!.value === "nova" );
  assert( wl.at(5)!.value === "shard" );
  assert( wl.at(6)!.value === "shard" );
  assert( wl.at(7)!.value === "shard" );
  assert( wl.at(8)!.value === "shard" );
  assert( wl.at(9)!.value === "shard" );
  assert( wl.at(10)!.value === "shard" );
  assert( wl.at(11)!.value === "shard" );
  assert( wl.at(12)        === undefined );
});

test("push", () => {
  assert.deepEqual(
    wl.clone().push({ weight: 13, value: "cortex" }).raw(),
    [
      [2, "sup"],
      [3, "nova"],
      [7, "shard"],
      [13, "cortex"],
    ]
  );

  assert.deepEqual( wl.clone().push("unit").raw(),
    [
      [2, "sup"],
      [3, "nova"],
      [7, "shard"],
      [1, "unit"],
    ]
  );
});
