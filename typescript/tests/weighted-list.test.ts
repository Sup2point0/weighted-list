import { WeightedList } from "../weighted-list/weighted-list";


const wl = new WeightedList(
  [2, "sup"],
  [3, "nova"],
  [7, "shard"],
);


test("constructors", () => {
  let test = WeightedList.from_object({
    sup: 2,
    nova: 3,
    shard: 7,
  });

  expect(test).toStrictEqual(wl);
});
