/**
 * Containers for weighted randomisation.
 * 
 * - Use `FrozenWeightedList` for storing a collection of values with associated weights.
 * - Use `WeightedList` (under development) if you wish to mutate the collection.
 * - Use `FrozenWeightedLists` to compose multiple separate `FrozenWeightedList`s into one larger pool.
 */

export * from "./shared";
// export * from "./weighted-list";
export * from "./frozen-weighted-list";
