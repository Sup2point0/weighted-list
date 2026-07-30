# Changelog


## Next (v0.2.0)

### `FrozenWeightedList`
- New: `is_zero()` method
- `.items()` correctly freezes returned objects
- Fix off-by-1 errors in indexing for sampling without replacement
- Improve error messages

### `WeightedCollection`
- Default `Item` to `WeightedItem<Value>`


## v0.1.5

### `FrozenWeightedList`
- Tighten invalid weight checks to error on $\text{NaN}$ and $\infin$ values
- Restructure type signature for `sample_values(_, options)` to disallow `decrement` when `replace: true`


## v0.1.4

- Fix incorrect export of members from `shared`


## v0.1.3

### `FrozenWeightedList`
- Fix incorrect usage of rounding that breaks non-integer weights
- Fix `RangeError` when `l.sample_values(count, { replace: false })` has `count > l.length`


## v0.1.2

### `FrozenWeightedList`
- Enforce stronger immutability with `Object.freeze` and `Readonly<>`
- Use `int` where appropriate in interface


## v0.1.1

### `FrozenWeightedList`
- Fix edge case errors in `.at()` indexing


## v0.1.0

- Initial release on npm!
