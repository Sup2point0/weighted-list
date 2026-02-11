# Changelog


## v0.6.0

### Breaking
- `WeightedList::select_random_values().unique()` option extracted to new `WeightedList::select_random_values_unique()` method
- `WeightedList::is_zero()` returns `true` instead of `false` when the list is empty

### New
- `WeightedList::select_random_values_unique()` method


## v0.5.0

### Breaking
- Removed `WeightedList::init()`
  - Use `::from()` (if possible), else `::from_iter(i)` or `i.collect()`
- `WeightedList::items()` returns `Vec<&WeightedItem>` instead of `&Vec<WeightedItem>`
- Relaxed trait bounds for `W: Weight`
  - `fmt::Display` changed to `fmt::Debug`

### New
- `WeightedList::collect_weights()` method
- `WeightedList::collect_values()` method
- Expose `Weight` trait
- Add `AsRef<>` conversions for `WeightedList`
- Improve `From<>` conversions for `WeightedList`

### Fixes
- Changed examples to use `u32` instead of `i32` to enforce non-negative weights
- Add missing documentation for crate items


## v0.4.1

### Fixes
- Add links to documentation
- Correct error in documentation for `WeightedList::is_zero`


## v0.4.0

### New
- `WList<V,W>` type alias for `WeightedList<V,W>`
- `WItem<V,W>` type alias for `WeightedItem<V,W>`
- `WeightedList` implements `FromIterator<(W,V)>`
- `WeightedList::from_expanded()` method
- `WeightedList::contains_value()` method
- `WeightedList::contains_weight()` method

### Fixes
- Cleanup and test other `From<>` implementations


## v0.3.1

### Fixes
- Fix `WeightedList::truncate()` implementation


## v0.3.0

### Breaking
- `WeightedList::remove()` renamed to `::remove_at()`
- `WeightedList::take_one()` renamed to `::take_one_at()`
- `WeightedList::take_by()` renamed to `::take_by_at()`
- `WeightedList::take_entire()` renamed to `::take_entire_at()`
- `WeightedList::take_by()` arguments reversed
  - Was `(weighted_index, decrement)`, now `(decrement, weighted_index)`
  - This aligns more intuitively with the expected order implied by `take_by_at` (take *by* `decrement` *at* `weighted_index`)

### New
- `WeightedList::remove_value_first()` method
- `WeightedList::remove_value_last()` method
- `WeightedList::remove_first_where()` method
- `WeightedList::remove_last_where()` method


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
- Fixed logic errors in random sampling methods


## v0.1.1

- Updated docs and README
- Added examples


## v0.1.0

- Intial release on crates.io!
