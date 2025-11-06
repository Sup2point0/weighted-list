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

  expect( test ).toStrictEqual( wl );
});

test("properties", () => {
  expect( wl.length ).toBe( 12 );
  expect( wl.total_weights ).toBe( 12 );
  expect( wl.total_values ).toBe( 3 );

  expect( wl.weights() ).toStrictEqual( [2, 3, 7] );
  expect( wl.values() ).toStrictEqual( ["sup", "nova", "shard"] );
  expect( wl.entries().toArray() ).toStrictEqual([
    [0, { weight: 2, value: "sup" }],
    [1, { weight: 3, value: "nova" }],
    [2, { weight: 7, value: "shard" }],
  ]);
  expect( wl.raw() ).toStrictEqual([
    [2, "sup"],
    [3, "nova"],
    [7, "shard"],
  ]);
});

test("at", () => {
  expect( wl.at(0)!.value ).toBe( "sup" );
  expect( wl.at(1)!.value ).toBe( "sup" );
  expect( wl.at(2)!.value ).toBe( "nova" );
  expect( wl.at(3)!.value ).toBe( "nova" );
  expect( wl.at(4)!.value ).toBe( "nova" );
  expect( wl.at(5)!.value ).toBe( "shard" );
  expect( wl.at(6)!.value ).toBe( "shard" );
  expect( wl.at(7)!.value ).toBe( "shard" );
  expect( wl.at(8)!.value ).toBe( "shard" );
  expect( wl.at(9)!.value ).toBe( "shard" );
  expect( wl.at(10)!.value ).toBe( "shard" );
  expect( wl.at(11)!.value ).toBe( "shard" );
  expect( wl.at(12) ).toBe( undefined );
});
