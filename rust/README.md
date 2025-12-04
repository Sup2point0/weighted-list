# weighted-list

A list implementation for weighted randomisation.


## Usage

Use in your crate:

```rust
use weighted_list::*;

// imports:
use weighted_list{
    WeightedList,
    wlist,
    WeightedItem,
    wit,
};
```

Use for weighted randomisation:

```rs
let descriptors = wlist![
    (10, String::from("cool")),
    (5, String::from("awesome")),
    (2,  String::from("elegant")),
    (1,  String::from("beautiful")),
];

let words = descriptors.select_random_values()
    .rng(&mut rand::rng())
    .count(2)
    .unique(true)
    .call();

println!("Rust is {} and {}", words[0], words[1]);
// => Rust is cool and elegant
```
