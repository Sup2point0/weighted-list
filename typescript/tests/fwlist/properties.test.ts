import { test, assert } from "vitest";

import { el, fwl } from "./shared";


test("length", () =>
{
  assert.equal( el.length,  0  );
  assert.equal( fwl.length, 10 );
});

test("total-weight", () =>
{
  assert.equal( el.total_weight,  0 );
  assert.equal( fwl.total_weight, 10 );
});

test("total-items", () =>
{
  assert.equal( el.total_items,  0 );
  assert.equal( fwl.total_items, 3 );
});

test("total-items", () =>
{
  assert.isTrue(  el.is_zero() );
  assert.isFalse( fwl.is_zero() );
});
