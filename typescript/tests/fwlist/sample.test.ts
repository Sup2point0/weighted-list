import { describe, test, assert } from "vitest";

import { FWList } from "../../weighted-list";
import { el, fwl } from "./shared";


const TRIALS = fwl().length * 2;
const EXPECTED = ["sup", "nova", "shard"];


describe("sample-value()", () =>
{
  test("returns value", () =>
  {
    for (let i = 0; i < TRIALS; i++) {
      let result = fwl().sample_value();
      assert.isDefined( result );
      assert.isTrue( EXPECTED.includes(result) );
    }
  });

  test("samples all values", () =>
  {
    let found = new Set();

    for (let i = 0; i < TRIALS; i++) {
      let result = fwl().sample_value();
      assert.isDefined(result);
      found.add(result);
    }

    assert.equal( found.size, 3 );
    assert.isTrue( found.has("sup") );
    assert.isTrue( found.has("nova") );
    assert.isTrue( found.has("shard") );
  });

  test("handles non-integer weights", () =>
  {
    {
      let l = new FWList(
        [0.5, "left"],
        [0.5, "right"],
      );

      let found = new Set();

      for (let i = 0; i < TRIALS; i++) {
        let result = l.sample_value();
        assert.isDefined(result);
        found.add(result);
      }

      assert.deepEqual( found, new Set(["left", "right"]) );
    }
    
    {
      let l = new FWList(
        [0.2, "p"],
        [0.3, "q"],
        [0.5, "r"],
      );

      let found = new Set();

      for (let i = 0; i < TRIALS; i++) {
        let result = l.sample_value();
        assert.isDefined(result);
        found.add(result);
      }

      assert.deepEqual( found, new Set(["p", "q", "r"]) );
    }
  });
});

describe("sample-values", () =>
{
  test("returns value", () =>
  {
    for (let i = 0; i < TRIALS; i++) {
      let results = fwl().sample_values(3);

      for (let value of results) {
        assert.isDefined( value );
        assert.include( EXPECTED, value );
      }
    }
  });
});

test("sample-values-without-replacement", () =>
{
  for (let i = 0; i < TRIALS; i++) {
    let results = fwl().sample_values(10, { replace: false });

    let counts: Record<string, number> = { "sup": 0, "nova": 0, "shard": 0 };

    for (let value of results) {
      counts[value!]++;
    }

    assert.equal( counts["sup"],   2 );
    assert.equal( counts["nova"],  3 );
    assert.equal( counts["shard"], 5 );
  }
});

test("sample-values-unique", () =>
{
  for (let i = 0; i < TRIALS; i++) {
    let results = fwl().sample_values_unique(10);
    let sorted = Array.from(results).toSorted((prot, deut) => prot.length - deut.length);

    assert.deepEqual( sorted, EXPECTED );
  }
  
  for (let i = 0; i < TRIALS; i++) {
    let pool = fwl().concat(fwl());
    // console.log("pool.at(0) =", pool.at(0));
    let results = pool.sample_values_unique(10);
    let sorted = Array.from(results).toSorted((prot, deut) => prot.length - deut.length);

    assert.deepEqual( sorted, EXPECTED.flatMap(each => [each, each]) );
  }
});

test("sample-values-unique-merging-duplicates", () =>
{
  for (let i = 0; i < TRIALS; i++) {
    let pool = fwl().concat(fwl());
    let results = pool.sample_values_unique(10, { merge_duplicates: true });
    let sorted = Array.from(results).toSorted((prot, deut) => prot.length - deut.length);

    assert.deepEqual( sorted, EXPECTED );
  }
});
