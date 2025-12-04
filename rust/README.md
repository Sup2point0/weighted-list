# weighted-list

A vector implementation for weighted randomisation.

Implements the `WeightedList` struct, an ordered collection of `WeightedItem`s, which hold a `value` and `weight`. Indexing and random selection takes the weight of items into consideration, such that items with greater weights are more likely to be selected.


## Usage

### Import
```rust
// blanket import
use weighted_list::*;

// specific imports
use weighted_list::{
    WeightedList, wlist,
    WeightedItem, wit,
};
```

### Construction
```rust
// macro
let wl = wlist![
    (2, "sup"),
    (3, "nova"),
    (5, "shard"),
];

// constructor
let wl = WeightedList::init(
    [
        (2, "sup"),
        (3, "nova"),
        (5, "shard"),
    ]
);
```

### Weighted Randomisation
```rs
let descriptors = wlist![
    (10, String::from("cool")),
    (5,  String::from("awesome")),
    (2,  String::from("elegant")),
    (1,  String::from("beautiful")),
];

// single selection
let word = descriptors.select_random_value(&mut rand::rng());

if let Some(chosen) = word {
    println!("You look {chosen}");
    // => You look cool
    //    (10/18 probability)
}

// multiple selection (bon builder syntax)
let words = descriptors.select_random_values()
    .rng(&mut rand::rng())
    .count(2)
    .unique(true)
    .call();

if let Some(first) = words[0] && let Some(second) = words[1] {
    println!("Rust is {} and {}", first, second);
    // => Rust is awesome and elegant
}
```

### Indexing
```rust
let wl = wlist![
    (1, "qi"),
    (2, "sup"),
    (5, "shard"),
];

wl[0]; // => WeightedItem { weight: 1, value: "qi" }
wl[1]; // => WeightedItem { weight: 2, value: "sup" }
wl[2]; // => WeightedItem { weight: 2, value: "sup" }
wl[3]; // => WeightedItem { weight: 5, value: "shard" }
wl[4]; // => WeightedItem { weight: 5, value: "shard" }
wl[5]; // => WeightedItem { weight: 5, value: "shard" }
wl[6]; // => WeightedItem { weight: 5, value: "shard" }
wl[7]; // => WeightedItem { weight: 5, value: "shard" }
wl[8]; // panic - out of bounds!
```
