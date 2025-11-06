type LikeWeightedItem<Value> = (
    WeightedItem<Value>
  | [number, any]
);


export interface WeightedItem<Value>
{
  weight: number;
  value: Value;
}


export class WeightedList<Value>
{
  #data: WeightedItem<Value>[];


  // == CONSTRUCTORS == //

  constructor(...items: LikeWeightedItem<Value>[])
  {
    this.#data = items.map(WeightedList.#sanitise);
    console.log("this.#data =", this.#data);
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

  get total_values() {
    return this.#data.length;
  }

  get total_length() {
    return this.length;
  }

  values(): Value[]
  {
    return this.#data.map(item => item.value);
  }

  weights(): number[]
  {
    return this.#data.map(item => item.weight);
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

    if ("value" in item && "weight" in item) {
      out = {
        weight: item.weight,
        value: item.value
      };
    }
    else {
      if (typeof item[Symbol.iterator] !== "function") {
        throw new TypeError("Invalid object format for WeightedItem");
      }

      if (item.length < 2) {
        throw new TypeError(
          `Expected 2 values in WeightedItem, but received ${item.length}`
        );
      }

      out = { weight: item[0], value: item[1] };
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

    return this;
  }
}


export default WeightedList;
