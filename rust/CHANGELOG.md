# Changelog


## v0.2.0

### Breaking
- `WeightedList::select_random_values()` takes a `usize` instead of `u32` for `count`

### New
- `WeightedItem` implements `Into<(W, V)>`
- `WeightedList::take_random_values()` method
- `WeightedList::take_random_values_unique()` method
- `WeightedList::expanded()` method
- `WeightedList::pruned()` method

### Fixes
- Fix logic errors in random sampling methods


## v0.1.1

- Updated docs and README
- Added examples


## v0.1.0

- Intial release on crates.io!
