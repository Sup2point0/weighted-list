import { FrozenWeightedList } from "../../src";
import type { int } from "../../src/shared";


/**
 * Construct an empty `FrozenWeightedList` for testing.
 */
export function el()
{
  return new FrozenWeightedList();
}

/**
 * Construct a non-empty `FrozenWeightedList` for testing.
 */
export function fwl()
{
  return new FrozenWeightedList(
    [2, "sup"],
    [3, "nova"],
    [5, "shard"],
  );
}

/**
 * Construct a `FrozenWeightedList` containing POJOs for mutability testing.
 */
export function ofwl()
{
  return new FrozenWeightedList(
    [1, { good: true }],
  );
}


/** Run `check` with different natural numbers for testing. */
export function for_any_natural(check: (n: int) => void)
{
  for (let i = 0; i < 7; i++) {
    check(i);
  }
}
