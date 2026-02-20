import { test, assert } from "vitest";

import { el, fwl } from "./shared";


const TRIALS = 20;
const EXPECTED = ["sup", "nova", "shard"];


test("sample-value", () =>
{
  for (let i = 0; i < TRIALS; i++) {
    let result = fwl().sample_value();
    assert.isTrue(EXPECTED.includes(result!));
  }
});

test("sample-values", () =>
{
  for (let i = 0; i < TRIALS; i++) {
    let results = fwl().sample_values(3);

    for (let value of results) {
      assert.isTrue(EXPECTED.includes(value!));
    }
  }
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
    console.log("pool.at(0) =", pool.at(0));
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
