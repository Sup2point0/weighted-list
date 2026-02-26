import type { Weight, WeightedItem, FrozenWeightedItem, LikeWeightedItem } from "./shared";


/**
 * An immutable list of weighted items.
 * 
 * "Immutable" means the items, the order of items, and the weights of items cannot be changed. However, their values can still be changed through obtaining a reference to the value of an item in the list.
 */
export class FrozenWeightedList<Value>
{
  #data: FrozenWeightedItem<Value>[];


  // == CONSTRUCTORS == //

  /**
   * Construct a `FrozenWeightedList` from the provided items.
   * 
   * @example
   * let fwl = new FrozenWeightedList(
   *   [2, "sup"],
   *   [3, "nova"],
   *   [5, "shard"],
   * );
   */
  constructor(...items: LikeWeightedItem<Value>[])
  {
    this.#data = [];
    let cumulative_weight = 0;

    for (let each of items) {
      let item = FrozenWeightedList.#sanitise<Value>(each, cumulative_weight);

      cumulative_weight = item.cumulative_weight;
      this.#data.push(item);
    }
  }


  // == ACCESSORS == //

  /** Get an iterator over the weights of each item in the list. */
  *iter_weights(): Generator<Weight>
  {
    for (let item of this.#data) {
      yield item.weight;
    }
  }

  /** Get an iterator over the values of each item in the list. */
  *iter_values(): Generator<Value>
  {
    for (let item of this.#data) {
      yield item.value;
    }
  }

  /** Get an iterator over the items in the list. */
  *iter_items(): Generator<WeightedItem<Value>>
  {
    for (let item of this.#data) {
      yield {
        weight: item.weight,
        value:  item.value
      };
    }
  }

  /** Get an iterator over the items in the list as `[index, item]` pairs. */
  *iter_entries(): Generator<[number, WeightedItem<Value>]>
  {
    for (let [i, item] of this.#data.entries()) {
      yield [
        i,
        {
          weight: item.weight,
          value:  item.value
        }
      ];
    }
  }

  /** Get an iterator over the items in the list as `[weight, value]` pairs. */
  *iter_raw(): Generator<[Weight, Value]>
  {
    for (let item of this.#data) {
      yield [item.weight, item.value];
    }
  }

  /** Get an iterator over the values in the list, with each value repeated a number of times equal to its weight (rounded up). */
  *iter_expanded(): Generator<Value>
  {
    for (let item of this.#data) {
      for (let i = 0; i < item.weight; i++) {
        yield item.value;
      }
    }
  }

  /** Get the weight of each item in the list. */
  weights(): Weight[]
  {
    return this.#data.map(item => item.weight);
  }

  /** Get the value of each item in the list. */
  values(): Value[]
  {
    return this.#data.map(item => item.value);
  }

  /**
   * Get the items in the list.
   * 
   * Note this returns shallow copies of the items - altering them will not alter the original list, but altering the values will alter the list.
   */
  items(): WeightedItem<Value>[]
  {
    return this.#data.map(item => ({
      weight: item.weight,
      value:  item.value,
    }));
  }

  /** Get the items in the list as `[index, item]` pairs. */
  entries(): [number, WeightedItem<Value>][]
  {
    return Array.from(this.items().entries());
  }

  /** Get the items in the list as `[weight, value]` pairs. */
  raw(): [Weight, Value][]
  {
    return this.#data.map(item => [item.weight, item.value]);
  }

  /** Get the values in the list, with each value repeated a number of times equal to its weight (rounded up). */
  expanded(): Value[]
  {
    return this.#data.flatMap(item => Array(Math.ceil(item.weight)).fill(item.value));
  }


  // == PROPERTIES == //

  /**
   * The total weight of all items in the list.
   */
  get length(): Weight {
    return this.#data.at(-1)?.cumulative_weight ?? 0;
  }

  /**
   * The total weight of all items in the list.
   * 
   * This may be preferred over `.length` when it could be perceived as ambiguous.
   */
  get total_weight(): Weight {
    return this.length;
  }

  /**
   * The total number of items in the list.
   */
  get total_items(): Weight {
    return this.#data.length;
  }


  // == INTERFACES == //

  /**
   * Get the item in the list at `weighted_index`.
   */
  at(weighted_index: Weight): WeightedItem<Value> | undefined
  {
    try {
      let item = this.#at(weighted_index);

      return {
        weight: item.weight,
        value:  item.value,
      };
    }
    catch {
      return undefined;
    }
  }



  // == ARRAY METHODS == //

  /** (out-of-place) Return this list concatenated with another `FrozenWeightedList`. */
  concat(other: FrozenWeightedList<Value>): FrozenWeightedList<Value>
  {
    return new FrozenWeightedList<Value>(...this.#data.concat(other.#data));
  }


  // == RANDOM SAMPLING == //

  /**
   * Randomly select 1 item from the list, using weighted randomisation. Returns `undefined` if the list is empty.
   */
  sample_item(): WeightedItem<Value> | undefined
  {
    if (this.length === 0) return undefined;

    let idx = this.#random_weighted_index();
    let out = this.at(idx)!;

    return out;
  }
  
  /**
   * Randomly select 1 value from the list, using weighted randomisation. Returns `undefined` if the list is empty.
   */
  sample_value(): Value | undefined
  {
    return this.sample_item()?.value;
  }

  /**
   * Randomly select `count` values from the list, using weighted randomisation.
   */
  *sample_values(
    count: number,
    options?: {
      /** Whether to select with replacement. Defaults to `true`. */
      replace: boolean;
      /** (only if `replace: false`) How much to decrement the weight of an item by after it is selected. Defaults to `1`. */
      decrement?: Weight;
    },
  ): Generator<Value | undefined>
  {
    options = Object.assign({
      replace:   true,
      decrement: 1,
    }, options);

    if (options.replace) {
      for (let n = 0; n < count; n++) {
        yield this.sample_value();
      }
    }
    else {
      let l = this.length;
      let weight_decrements = Array(this.total_items).fill(0);

      for (let n = 0; n < count; n++)
      {
        let widx = this.#random_weighted_index_up_to(l);
        let  idx = this.#unweight_index_decrementing(widx, weight_decrements);

        let target = this.#data.at(idx)!;

        if (target.weight < options.decrement!) {
          weight_decrements[idx] += target.weight;
          l -= target.weight;
        }
        else {
          weight_decrements[idx] += options.decrement!;
          l -= options.decrement!;
        }

        yield target.value;
      }
    }
  }

  /**
   * Randomly select `count` unique values from the list, using weighted randomisation.
   * @param count How many values to select. The generator will yield this many values *at most*.
   */
  *sample_values_unique(
    count: number,
    options?: {
      /** By default, each item in the list is treated as a 'unique' value. If `merge_duplicates` is set to `true`, duplicate values (values that compare `===`) will be treated as equivalent. */
      merge_duplicates: boolean,
    },
  ): Generator<Value>
  {
    let seen_indices = new Set<number>();
    let l = this.length;

    for (let n = 0; n < count; n++)
    {
      if (l <= 0) break;
      
      let weighted_index = Math.floor(Math.random() * l);
      let idx = this.#unweight_index_skipping(weighted_index, seen_indices)
      let out = this.#data[idx];

      if (options?.merge_duplicates) {
        for (let [i, item] of this.#data.entries()) {
          if (item.value === out.value) {
            seen_indices.add(i);
            l -= item.weight;
          }
        }
      }
      else {
        seen_indices.add(idx);
        l -= out.weight;
      }

      yield out.value;
    }
  }


  // == INTERNAL == //

  /** Convert a compatible object into a `FrozenWeightedItem`, throwing an error if the conversion fails. */
  static #sanitise<Value>(
    item: LikeWeightedItem<Value>,
    cumulative_weight: Weight,
  ): FrozenWeightedItem<Value>
  {
    let out: FrozenWeightedItem<Value>;
    
    if (
      typeof item === "object"
      && "value" in item
      && "weight" in item
    ) {
      out = {
        cumulative_weight: cumulative_weight + item.weight,
        weight: item.weight,
        value: item.value
      };
    }
    else if (typeof item[Symbol.iterator] === "function" && typeof item !== "string") {
      if (item.length != 2) {
        throw new TypeError(
          `Expected 2 values in \`FrozenWeightedItem\`, but received ${item.length} values`
        );
      }

      try {
        out = {
          cumulative_weight: cumulative_weight + item[0],
          weight: item[0],
          value: item[1]
        };
      }
      catch {
        throw new TypeError("Invalid object format for `FrozenWeightedItem`");
      }
    }
    else {
      out = {
        cumulative_weight: cumulative_weight + 1,
        weight: 1,
        value: item as Value
      };
    }

    return this.#check(out);
  }
  
  /** Check if `item` is a valid `FrozenWeightedItem`. */
  static #check<Value>(item: FrozenWeightedItem<Value>): FrozenWeightedItem<Value>
  {
    if (typeof item.weight !== "number") {
      throw new TypeError(
        `Expected numeric type for item weight, but received ${item.weight} of type <${typeof item.weight}>.`
        + (typeof item.value === "number") ? " Perhaps you got the value and weight the wrong way round? (weight always comes first)" : ""
      );
    }

    if (item.weight <= 0) {
      throw new Error(
        `Received invalid \`FrozenWeightedItem\`: ${item} - weight must be positive`
      );
    }

    return item;
  }

  /** Convert a weighted index to its corresponding unweighted index in the list, using binary search. */
  #binary_unweight_index(weighted_index: Weight): Weight
  {
    let max = this.total_items;

    if (max !== 0)
    {
      const cycles = Math.floor(Math.log2(max) + 1);

      let l = 0;
      let r = max - 1;

      for (let i = 0; i < cycles; i++) {
        let idx = Math.floor(l + (r - l) / 2);

        let cand = this.#data[idx];
        let c_weight = cand.cumulative_weight;

        if (c_weight > weighted_index && weighted_index >= c_weight - cand.weight) {
          return idx;
        }

        if (weighted_index < c_weight) {
          r = idx - 1;
        } else {
          l = idx + 1;
        }
      }
    }

    throw new RangeError(
      `Attempted to access weighted index ${weighted_index}, but \`FrozenWeightedList\` has weighted length ${this.length}`
    );
  }

  /** Convert a weighted index to its corresponding unweighted index in the list, using linear search, skipping a set of seen (unweighted) indices. */
  #unweight_index_skipping(weighted_index: Weight, seen_indices: Set<number>): Weight
  {
    let t = 0;

    for (let [i, item] of this.#data.entries()) {
      if (seen_indices.has(i)) continue;

      t += item.weight;
      if (t >= weighted_index) return i;
    }

    throw new RangeError(
      `Attempted to access weighted index ${weighted_index}, skipping indices ${seen_indices}, but went out of bounds`
    );
  }

  /** Convert a weighted index to its corresponding unweighted index in the list, using linear search, applying a map of weight decrements. */
  #unweight_index_decrementing(weighted_index: Weight, weight_decrements: Weight[]): Weight
  {
    let t = 0;

    let w: Weight;

    for (let [i, item] of this.#data.entries()) {
      w = item.weight - weight_decrements[i];

      if (w <= 0) continue;

      t += w;
      if (t >= weighted_index) return i;
    }

    throw new RangeError(
      `Attempted to access weighted index ${weighted_index}, with decrements ${weight_decrements}, but went out of bounds`
    );
  }

  /** Get a random weighted index in the list. */
  #random_weighted_index(): Weight
  {
    return this.#random_weighted_index_up_to(this.length);
  }

  /** Get a random weighted index in the list, up to but excluding `length`. */
  #random_weighted_index_up_to(length: Weight): Weight
  {
    return Math.floor(Math.random() * length);
  }

  /** Get the item at `weighted_index` in the list. */
  #at(weighted_index: Weight): FrozenWeightedItem<Value>
  {
    if (weighted_index < 0) {
      throw new RangeError(`Attempted to index \`WeightedList\` at ${weighted_index} - negative indices are currently unsupported!`);
    }

    let out = this.#data[this.#binary_unweight_index(weighted_index)];
    if (out !== undefined) return out;

    throw new RangeError(
      `Attempted to access weighted index ${weighted_index}, but \`WeightedList\` has weighted length ${this.length}`
    );
  }
}

export default FrozenWeightedList
