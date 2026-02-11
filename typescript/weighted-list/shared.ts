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
