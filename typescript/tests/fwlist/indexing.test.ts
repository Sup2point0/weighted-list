import { test, assert } from "vitest";

import { el, fwl } from "./shared";
import { FrozenWeightedList } from "../../weighted-list";


test("at-empty", () =>
{
  assert.equal( el().at(0), undefined );
});

test("at-negative-unsupported", () =>
{
  assert.equal( fwl().at(-1), undefined );
});

test("at-standard", () =>
{
  assert.equal( fwl().at(0)!.value, "sup"   );
  assert.equal( fwl().at(1)!.value, "sup"   );
  assert.equal( fwl().at(2)!.value, "nova"  );
  assert.equal( fwl().at(3)!.value, "nova"  );
  assert.equal( fwl().at(4)!.value, "nova"  );
  assert.equal( fwl().at(5)!.value, "shard" );
  assert.equal( fwl().at(6)!.value, "shard" );
  assert.equal( fwl().at(7)!.value, "shard" );
  assert.equal( fwl().at(8)!.value, "shard" );
  assert.equal( fwl().at(9)!.value, "shard" );
  assert.equal( fwl().at(12), undefined );
});

test("at-small", () =>
{
  const fwl = new FrozenWeightedList([1, "qi"]);
  
  assert.equal( fwl.at(0)!.value, "qi");
  assert.equal( fwl.at(1), undefined);
});

test("at-units", () =>
{
  const fwl = new FrozenWeightedList(
    [1, "sup"],
    [1, "nova"],
    [1, "shard"],
    [1, "cortex"],
    [1, "origin"],
    [1, "vision"],
  );
  
  assert.equal( fwl.at(0)!.value, "sup");
  assert.equal( fwl.at(1)!.value, "nova");
  assert.equal( fwl.at(2)!.value, "shard");
  assert.equal( fwl.at(3)!.value, "cortex");
  assert.equal( fwl.at(4)!.value, "origin");
  assert.equal( fwl.at(5)!.value, "vision");
});

test("at-large", () =>
{
  const fwl = new FrozenWeightedList(
    [2, "sup"],
    [3, "nova"],
    [5, "shard"],
    [7, "cortex"],
    [13, "origin"],
    [20, "vision"],
  );

  assert.equal( fwl.at(0)!.value,  "sup"   );
  assert.equal( fwl.at(1)!.value,  "sup"   );
  assert.equal( fwl.at(2)!.value,  "nova"  );
  assert.equal( fwl.at(3)!.value,  "nova"  );
  assert.equal( fwl.at(4)!.value,  "nova"  );
  assert.equal( fwl.at(5)!.value,  "shard" );
  assert.equal( fwl.at(6)!.value,  "shard" );
  assert.equal( fwl.at(7)!.value,  "shard" );
  assert.equal( fwl.at(8)!.value,  "shard" );
  assert.equal( fwl.at(9)!.value,  "shard" );
  assert.equal( fwl.at(10)!.value, "cortex" );
  assert.equal( fwl.at(11)!.value, "cortex" );
  assert.equal( fwl.at(12)!.value, "cortex" );
  assert.equal( fwl.at(13)!.value, "cortex" );
  assert.equal( fwl.at(14)!.value, "cortex" );
  assert.equal( fwl.at(15)!.value, "cortex" );
  assert.equal( fwl.at(16)!.value, "cortex" );
  assert.equal( fwl.at(17)!.value, "origin" );
  assert.equal( fwl.at(18)!.value, "origin" );
  assert.equal( fwl.at(19)!.value, "origin" );
  assert.equal( fwl.at(20)!.value, "origin" );
  assert.equal( fwl.at(21)!.value, "origin" );
  assert.equal( fwl.at(22)!.value, "origin" );
  assert.equal( fwl.at(23)!.value, "origin" );
  assert.equal( fwl.at(24)!.value, "origin" );
  assert.equal( fwl.at(25)!.value, "origin" );
  assert.equal( fwl.at(26)!.value, "origin" );
  assert.equal( fwl.at(27)!.value, "origin" );
  assert.equal( fwl.at(28)!.value, "origin" );
  assert.equal( fwl.at(29)!.value, "origin" );
  assert.equal( fwl.at(30)!.value, "vision" );
  assert.equal( fwl.at(31)!.value, "vision" );
  assert.equal( fwl.at(32)!.value, "vision" );
  assert.equal( fwl.at(33)!.value, "vision" );
  assert.equal( fwl.at(34)!.value, "vision" );
  assert.equal( fwl.at(35)!.value, "vision" );
  assert.equal( fwl.at(36)!.value, "vision" );
  assert.equal( fwl.at(37)!.value, "vision" );
  assert.equal( fwl.at(38)!.value, "vision" );
  assert.equal( fwl.at(39)!.value, "vision" );
  assert.equal( fwl.at(40)!.value, "vision" );
  assert.equal( fwl.at(41)!.value, "vision" );
  assert.equal( fwl.at(42)!.value, "vision" );
  assert.equal( fwl.at(43)!.value, "vision" );
  assert.equal( fwl.at(44)!.value, "vision" );
  assert.equal( fwl.at(45)!.value, "vision" );
  assert.equal( fwl.at(46)!.value, "vision" );
  assert.equal( fwl.at(47)!.value, "vision" );
  assert.equal( fwl.at(48)!.value, "vision" );
  assert.equal( fwl.at(49)!.value, "vision" );
});
