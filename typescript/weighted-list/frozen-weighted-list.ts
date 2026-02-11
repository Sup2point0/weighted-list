import type { Weight, WeightedItem, FrozenWeightedItem, LikeWeightedItem } from "./shared";


export class FrozenWeightedList<Value>
{
  #data: FrozenWeightedItem<Value>[];

  // == CONSTRUCTORS == //
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
  weights(): Weight[]
  {
    return this.#data.map(item => item.weight);
  }

  values(): Value[]
  {
    return this.#data.map(item => item.value);
  }

  items(): WeightedItem<Value>[]
  {
    return this.#data.map(item => ({
      weight: item.weight,
      value:  item.value,
    }))
  }

  entries(): ArrayIterator<[number, WeightedItem<Value>]>
  {
    return this.items().entries();
  }

  *raw(): Generator<[Weight, Value]>
  {
    for (let item of this.#data) {
      yield [item.weight, item.value];
    }
  }

  *expanded(): Generator<Value>
  {
    for (let item of this.#data) {
      for (let i = 0; i < item.weight; i++) {
        yield item.value;
      }
    }
  }

  // == PROPERTIES == //
  get length(): Weight {
    return this.#data.at(-1)?.cumulative_weight ?? 0;
  }

  get total_weights(): Weight {
    return this.length;
  }

  get total_values(): Weight {
    return this.#data.length;
  }

  is_zero(): boolean {
    return (this.length === 0);
  }

  // == INDEXING == //
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

  // == SPECIALISED == //
  sample_value(): WeightedItem<Value> | undefined
  {
    if (this.is_zero()) return undefined;

    let idx = Math.floor(Math.random() * this.length);
    let out = this.at(idx)!;

    return out;
  }

  *sample_values(
    count: number,
    options?: {
      replace?: boolean;
      decrement?: Weight;
    },
  ): Generator<Value>
  {
    let pool = structuredClone(this);

    for (let n = 0; n < count; n++)
    {
      if (pool.length <= 0) break;

      let idx = Math.floor(Math.random() * pool.length);
      let out = pool.at(idx)!;

      yield out.value;
    }
  }

  *sample_values_unique(
    count: number,
    options?: {
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
      let out = this.at(idx)!;

      if (options?.merge_duplicates) {
        for (let [i, item] of this.#data.entries()) {
          if (item.value != out.value) continue;

          seen_indices.add(i);
          l -= item.weight;
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
        cumulative_weight:
          "cumulative_weight" in item
          ? item.cumulative_weight
          : cumulative_weight + item.weight,
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
  
  static #check<Value>(item: FrozenWeightedItem<Value>): FrozenWeightedItem<Value>
  {
    if (typeof item.weight !== "number") {
      throw new TypeError(
        `Expected numeric type for item weight, but received ${item.weight} of type <${typeof item.weight}>.`
        + (typeof item.value === "number") ? " Perhaps you got the value and weight the wrong way round? (weight always comes first)" : ""
      );
    }

    if (item.weight <= 0) {
      if (item.weight === 0) {
        console.warn(
          `Received a \`FrozenWeightedItem\` with zero weight: ${item}`
        );
      }

      throw new Error(
        `Received invalid \`FrozenWeightedItem\`: ${item} - weight of a cannot be negative`
      );
    }

    return item;
  }

  #binary_unweight_index(weighted_index: Weight): Weight
  {
    let max = this.total_values;

    if (max !== 0)
    {
      const cycles = Math.ceil(Math.log2(max));

      let l = 0;
      let r = max - 1;

      for (let i = 0; i < cycles; i++) {
        let idx = l + (r - l) / 2;

        let cand = this.#data[idx];
        let weight = cand.weight;
        let c_weight = cand.cumulative_weight;

        if (c_weight > weighted_index && weighted_index >= c_weight - weight) {
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

  #unweight_index_skipping(weighted_index: Weight, seen_indices: Set<number>): Weight
  {
    let t = 0;

    for (let [i, item] of this.#data.entries()) {
      if (seen_indices.has(i)) continue;

      t += item.weight;
      if (t >= weighted_index) return i;
    }

    throw new RangeError(
      `Attempted to access weighted index ${weighted_index}, skipping indices ${seen_indices}, but \`WeightedList\` has weighted length ${this.length}`
    );
  }

  #at(weighted_index: Weight): FrozenWeightedItem<Value>
  {
    let out = this.#data[this.#binary_unweight_index(weighted_index)];
    if (out !== undefined) return out;

    throw new RangeError(
      `Attempted to access weighted index ${weighted_index}, but \`WeightedList\` has weighted length ${this.length}`
    );
  }
}

export default FrozenWeightedList
