import type { int, Weight, WeightedItem, FrozenWeightedItem, LikeWeightedItem } from "./shared";
import { FrozenWeightedList } from "./frozen-weighted-list";


/**
 * A composite immutable list of weight items, with multiple backing `FrozenWeightedList`s.
 */
export class FrozenWeightedLists<Value>
{
  #lists: FrozenWeightedList<Value>[];
  #length: Weight;

  
  constructor(...lists: FrozenWeightedList<Value>[])
  {
    this.#lists = lists;
    this.#length = lists.reduce((total, list) => total + list.length, 0);
  }


  // == ACCESSORS == //

  /** Get an iterator over the weights of each item in the list. */
  *iter_weights(): Generator<Weight>
  {
    for (let list of this.#lists) {
      yield* list.iter_weights();
    }
  }

  /** Get an iterator over the values of each item in the list. */
  *iter_values(): Generator<Value>
  {
    for (let list of this.#lists) {
      yield* list.iter_values();
    }
  }
  
  /** Get an iterator over the items in the list. */
  *iter_items(): Generator<Readonly<WeightedItem<Value>>>
  {
    for (let list of this.#lists) {
      yield* list.iter_items();
    }
  }

  /** Get an iterator over the items in the list as `[index, item]` pairs. */
  *iter_entries(): Generator<[int, Readonly<WeightedItem<Value>>]>
  {
    for (let list of this.#lists) {
      yield* list.iter_entries();
    }
  }

  /** Get an iterator over the items in the list as `[weight, value]` pairs. */
  *iter_raw(): Generator<[Weight, Value]>
  {
    for (let list of this.#lists) {
      yield *list.iter_raw();
    }
  }

  /** Get an iterator over the values in the list, with each value repeated a number of times equal to its weight (rounded up). */
  *iter_expanded(): Generator<Value>
  {
    for (let list of this.#lists) {
      yield *list.iter_expanded();
    }
  }

  /** Get the weight of each item in the list. */
  weights(): Weight[]
  {
    return this.#lists.flatMap(list => list.weights());
  }

  /** Get the value of each item in the list. */
  values(): Value[]
  {
    return this.#lists.flatMap(list => list.values());
  }

  /**
   * Get the items in the list as a plain array.
   */
  items(): Readonly<WeightedItem<Value>>[]
  {
    return this.#lists.flatMap(list => list.items());
  }

  /** Get the items in the list as `[index, item]` pairs. */
  entries(): [int, Readonly<WeightedItem<Value>>][]
  {
    return Array.from(this.items().entries());
  }

  /** Get the items in the list as `[weight, value]` pairs. */
  raw(): [Weight, Value][]
  {
    return this.#lists.flatMap(list => list.raw());
  }

  /** Get the values in the list, with each value repeated a number of times equal to its weight (rounded up). */
  expanded(): Value[]
  {
    return this.#lists.flatMap(list => list.expanded());
  }


  // == PROPERTIES == //

  /**
   * The total weight of all items in the list.
   */
  get length(): Weight {
    return this.#length;
  }

  /**
   * The total weight of all items in the list.
   * 
   * This may be preferable over `.length` when it could be perceived as ambiguous.
   */
  get total_weight(): Weight {
    return this.length;
  }

  /**
   * The total number of items in the list.
   */
  get total_items(): Weight {
    return this.#lists.reduce((total, list) => total + list.total_items, 0);
  }
}
