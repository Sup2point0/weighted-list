import { test, assert } from "vitest";

import { el, fwl } from "./shared";


const TRIALS = 42;


test("sample-value", () =>
{
  const expected = ["sup", "nova", "shard"];

  for (let i = 0; i < TRIALS; i++) {
    let result = fwl.sample_value();
    assert.isTrue(expected.includes(result!));
  }
});

test("sample-values", () =>
{
  const expected = ["sup", "nova", "shard"];

  for (let i = 0; i < TRIALS; i++) {
    let results = fwl.sample_values(3);

    for (let value of results) {
      assert.isTrue(expected.includes(value!));
    }
  }
});

test("sample-values-without-replacement", () =>
{
  const expected = ["sup", "nova", "shard"];

  for (let i = 0; i < TRIALS; i++) {
    let results = fwl.sample_values(10, { replace: false });

    let counts: Record<string, number> = { "sup": 0, "nova": 0, "shard": 0 };

    for (let value of results) {
      counts[value!]++;
    }

    assert.equal( counts["sup"],   2 );
    assert.equal( counts["nova"],  3 );
    assert.equal( counts["shard"], 5 );
  }
});
