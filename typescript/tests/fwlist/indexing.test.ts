import { test, assert } from "vitest";

import { el, fwl } from "./shared";


test("at-empty", () =>
{
  assert.equal( el.at(0), undefined );
});

test("at-standard", () =>
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

test("at-negative-unsupported", () =>
{
  assert.equal( fwl.at(-1), undefined );
});
