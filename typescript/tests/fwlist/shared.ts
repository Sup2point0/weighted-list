import { FrozenWeightedList } from "../../weighted-list";


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
