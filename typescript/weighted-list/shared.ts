export type int = number;
export type Weight = number;


/**
 * An item in a `WeightedList`.
 */
export interface WeightedItem<Value>
{
  weight: Weight;
  value: Value;
}

/**
 * An item in a `FrozenWeightedList`.
 */
export interface FrozenWeightedItem<Value> extends WeightedItem<Value>
{
  cumulative_weight: Weight;
  weight: Weight,
  value: Value,
}


/** Any value that could be converted to a `WeightedItem`. */
export type LikeWeightedItem<Value> = (
    [Weight, Value]
  | WeightedItem<Value>
  | FrozenWeightedItem<Value>
);


/**
 * The standard baseline interface all weighted collections will implement. This allows easily migrating between, for instance, a `WeightedList` and `FrozenWeightedList`.
 */
export interface WeightedCollection<Value, Item>
{
  // == ACCESSORS == //
  iter_weights():  Generator<Weight>
  iter_values():   Generator<Value>
  iter_items():    Generator<Item>
  iter_entries():  Generator<[int, Item]>
  iter_raw():      Generator<[Weight, Value]>
  iter_expanded(): Generator<Value>

  weights():  Weight[]
  values():   Value[]
  items():    Item[]
  entries():  [int, Item][]
  raw():      [Weight, Value][]
  expanded(): Value[]

  // == PROPERTIES == //
  get length():       Weight
  get total_weight(): Weight
  get total_items():  Weight

  // == ARRAY METHODS == //
  at(weighted_index: Weight): Readonly<WeightedItem<Value>> | undefined

  // == SAMPLING == //
  sample_item(): WeightedItem<Value> | undefined
  sample_value(): Value | undefined
  sample_values(count: int, options?: object): Generator<Value | undefined>
  sample_values_unique(count: int, options?: object): Generator<Value>
}
