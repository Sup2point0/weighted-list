import { FrozenWeightedList } from "../../weighted-list";


export const FWList = FrozenWeightedList;


export const el = new FrozenWeightedList();

export const fwl = new FrozenWeightedList(
  [2, "sup"],
  [3, "nova"],
  [5, "shard"],
);
