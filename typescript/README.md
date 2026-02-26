# weighted-list

Data structures for weighted randomisation.

This library provides the `WeightedList` (under development) and `FrozenWeightedList` classes for storing weighted items. When randomly selecting items, those with a higher weight are more likely to be chosen.


## Features

- Weighted indexing
- Random sampling methods with support for unique only, with/without replacement, multiple decrements

### Use Cases
- Randomised messages
- Loot/rewards in games
- Inventory systems
- Easter eggs


## Usage

### Import
```ts
import {
  FrozenWeightedList,
  FrozenWeightedItem,
} from "@sup2.0/weighted-list";
```

### Construction
```ts
const fwl = new FrozenWeightedList(
  [2, "sup"],
  [3, "nova"],
  [5, "shard"],
);
```

### Weighted Sampling
```ts
const descriptors = new FrozenWeightedList(
  [10, "cool"],
  [5,  "awesome"],
  [2,  "fun"],
);

// single selection
let word = descriptors.sample_value();

console.log(`You look ${word}`);
// => You look cool
//    (10/18 probability)

// multiple selection
let words = descriptors.sample_unique_values();

console.log(`TypeScript is ${words[0]} and ${words[1]}`);
// => TypeScript is awesome and fun
```

### Weighted Indexing
```ts
const fwl = new FrozenWeightedList([2, "sup"], [3, "nova"], [5, "shard"]);

fwl.at(0) // => { weight: 2, value: "sup" }
fwl.at(1) // => { weight: 2, value: "sup" }
fwl.at(2) // => { weight: 3, value: "nova" }
fwl.at(3) // => { weight: 3, value: "nova" }
fwl.at(4) // => { weight: 3, value: "nova" }
fwl.at(5) // => { weight: 5, value: "shard" }
fwl.at(6) // => { weight: 5, value: "shard" }
fwl.at(7) // => { weight: 5, value: "shard" }
fwl.at(8) // => { weight: 5, value: "shard" }
fwl.at(9) // => { weight: 5, value: "shard" }
fwl.at(10) // => undefined - out of bounds!
```


## Changelog

[View on GitHub](https://github.com/Sup2point0/weighted-list/blob/main/typescript/CHANGELOG.md).
