export type Weight = number;


/**
 * An item in a `WeightedList`.
 */
export interface WeightedItem<Value>
{
  weight: Weight;
  value: Value;
}


export type LikeWeightedItem<Value> = (
    WeightedItem<Value>
  | [Weight, Value]
);
