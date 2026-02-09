import type { Weight, WeightedItem, LikeWeightedItem } from "./shared";


export type LikeFrozenWeightedItem<Value> = (
    FrozenWeightedItem<Value>
  | LikeWeightedItem<Value>
);


export interface FrozenWeightedItem<Value> extends WeightedItem<Value>
{
  cumulative_weight: Weight;
  weight: Weight,
  value: Value,
}

export class FrozenWeightedList<Value>
{
  #data: FrozenWeightedItem<Value>[];

  // == CONSTRUCTORS == //
  constructor(...items: LikeFrozenWeightedItem<Value>[])
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

  entries(): ArrayIterator<[Weight, WeightedItem<Value>]>
  {
    return this.items().entries();
  }

  raw(): [Weight, Value][]
  {
    return this.#data.map(item => [item.weight, item.value]);
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

  // ==  == //
  at(weighted_index: Weight): FrozenWeightedItem<Value> | undefined
  {
    try {
      return this.#at(weighted_index);
    }
    catch {
      return undefined;
    }
  }

  // == INTERNAL == //
  static #sanitise<Value>(
    item: LikeFrozenWeightedItem<Value>,
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
    if (item.weight < 0) {
      throw new Error("Weight of a `FrozenWeightedItem` cannot be negative");
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
      `Attempted to access weighted index ${weighted_index} but \`FrozenWeightedList\` has weighted length ${this.length}`
    );
  }

  #at(weighted_index: Weight): FrozenWeightedItem<Value>
  {
    let out = this.#data[this.#binary_unweight_index(weighted_index)];
    if (out !== undefined) return out;

    throw new RangeError(`Attempted to access weighted index ${weighted_index} but WeightedList has weighted length ${this.length}`);
  }
}
