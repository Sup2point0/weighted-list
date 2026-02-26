import { FrozenWeightedList } from "../../weighted-list";


export const FWList = FrozenWeightedList;


export function el()
{
  return new FrozenWeightedList();
}

export function fwl()
{
  return new FrozenWeightedList(
    [2, "sup"],
    [3, "nova"],
    [5, "shard"],
  );
}
