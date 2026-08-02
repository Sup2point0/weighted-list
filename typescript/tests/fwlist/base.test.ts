import { describe, test, assert } from "vitest";

import { FWList } from "../../src";
import { el, fwl } from "./shared";


describe("Symbol.iterator", () =>
{
  test("usual", () => {
    let count = 0; let expected_count = fwl().total_items;
    let total = 0; let expected_total = fwl().total_weight;

    for (let item of fwl()) {
      count++;
      total += item.weight;
    }

    assert.equal( count, expected_count );
    assert.equal( total, expected_total );
  });

  test("usual", () => {
    let count = 0;
    let total = 0;

    for (let item of el()) {
      count++;
      total += item.weight;
    }

    assert.equal( count, 0 );
    assert.equal( total, 0 );
  });

  test("cannot mutate", () => {
    let l = fwl();
    let before = l.total_weight;

    for (let item of l) {
      assert.throws(() => {
        // if type-check error, good - list should be immutable
        item.value = null;
        item.weight++;
      });
    }

    assert.equal( l.total_weight, before );
  });
});
