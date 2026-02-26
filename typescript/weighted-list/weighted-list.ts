import type { WeightedItem, LikeWeightedItem } from "./shared";


export class WeightedList<Value>
{
  #data: WeightedItem<Value>[];


  // == CONSTRUCTORS == //
  constructor(...items: LikeWeightedItem<Value>[])
  {
    this.#data = items.map(WeightedList.#sanitise);
  }

  /**
   * Construct a `WeightedList` from a provided object, where `key: value` pairs become `{ value: key, weight: value }` objects.
   * 
   * ```ts
   * >>> WeightedList.from_object({ sup: 2, nova: 3 })
   * WeightedList [ { value: "sup", weight: 2 },
   *                { value: "nova", weight: 3 } ]
   * ```
   */
  static from_object(list: object)
  {
    let out = new WeightedList();
    
    out.#data = Object.entries(list).map(
      ([value, weight]) => WeightedList.#check({ weight, value })
    );

    return out;
  }


  // == PROPERTIES == //
  get length() {
    return this.#data.reduce((acc, item) => acc + item.weight, 0);
  }

  get total_weights() {
    return this.length;
  }

  get total_values() {
    return this.#data.length;
  }

  weights(): number[]
  {
    return this.#data.map(item => item.weight);
  }

  values(): Value[]
  {
    return this.#data.map(item => item.value);
  }

  entries(): ArrayIterator<[number, WeightedItem<Value>]>
  {
    return this.#data.entries();
  }

  raw(): [number, Value][]
  {
    return this.#data.map(item => [item.weight, item.value]);
  }


  // == INTERNAL == //

  static #sanitise<Value>(item: LikeWeightedItem<Value>): WeightedItem<Value>
  {
    let out: WeightedItem<Value>;

    if (typeof item === "object" && "value" in item && "weight" in item) {
      out = {
        weight: item.weight,
        value: item.value
      };
    }
    else if (typeof item[Symbol.iterator] === "function" && typeof item !== "string") {
      if (item.length < 2) {
        throw new TypeError(
          `Expected 2 values in WeightedItem, but received ${item.length}`
        );
      }

      try {
        out = { weight: item[0], value: item[1] };
      }
      catch {
        throw new TypeError("Invalid object format for WeightedItem");
      }
    }
    else {
      out = { weight: 1, value: item as Value };
    }

    return WeightedList.#check(out);
  }

  static #check<Value>(item: WeightedItem<Value>): WeightedItem<Value>
  {
    if (typeof item.weight !== "number") {
      throw new TypeError(
        `Expected numeric type for item weight, but received ${item.weight} of type <${typeof item.weight}>.`
        + (typeof item.value === "number") ? " Perhaps you got the value and weight the wrong way round? (Weight comes first)" : ""
      );
    }
    if (item.weight < 0) {
      throw new Error("Item weight cannot be negative");
    }

    return item;
  }

  #at(weighted_index: number): WeightedItem<Value>
  {
    let t = 0;

    for (let item of this.#data) {
      t += item.weight;

      if (t > weighted_index) {
        return item;
      }
    }

    throw new RangeError(`Attempted to access weighted index ${weighted_index} but WeightedList has weighted length ${this.length}`);
  }


  // == CORE == //

  /**
   * Create a clone of the `WeightedList` and the `WeightedItem`s it contains. Values stored in the `WeightedItem`s are not cloned.
   */
  clone(): WeightedList<Value>
  {
    /* @ts-ignore */
    return new WeightedList(
      ...this.#data.map(
        /* @ts-ignore */
        ({ weight, value }) => ({ weight, value })
      )
    );
  }


  // == ARRAY METHODS == //

  at(weighted_index: number): WeightedItem<Value> | undefined
  {
    try {
      return this.#at(weighted_index)
    }
    catch {
      return;
    }
  }

  push(
    value: Value | WeightedItem<Value>,
    weight?: number
  ): WeightedList<Value>
  {
    if (weight === undefined) {
      this.#data.push(WeightedList.#sanitise(value as WeightedItem<Value>));
    }
    else {
      this.#data.push({ weight, value: value as Value });
    }

    console.log("this =", this);

    return this;
  }

  clear(): WeightedList<Value>
  {
    this.#data = [];
    return this;
  }


  // == SPECIALIST METHODS == //

  /**
   * Randomly select a single `WeightedItem` from the list using weighted randomisation.
   * 
   * @returns A randomly selected `WeightedItem`. Use `.value` to access its value.
   */
  select_single(): WeightedItem<Value>
  {
    let idx = Math.floor(Math.random() * this.length);
    let out = this.at(idx)!;

    return out;
  }

  *select_random(
    count: number = 1,
    options?: WeightedSelectionOptions
  ): Generator<WeightedItem<Value>>
  {
    const DEFAULTS = {
      replace: true,
      entire: false,
      unique: false,
    };

    options = Object.assign(DEFAULTS, options);

    let out: WeightedItem<Value>;
    let list = this.clone();

    for (let i = 0; i < count; i++)
    {
      if (list.length < 1) {
        break;
      }

      let idx = Math.floor(Math.random() * this.length);
      out = this.at(idx)!;

      if (options.unique) {
        throw new Error("Not implemented yet!");
      }
      else if (!options.replace) {
        out.weight -= 1;
      }

      yield out;
    }
  }
}


interface WeightedSelectionOptions {
  entire?: boolean;
  replace?: boolean;
  unique?: boolean
}


export default WeightedList;
