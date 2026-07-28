# Changelog


## v0.1.3

### `FrozenWeightedList`
- Fix incorrect usage of rounding that breaks non-integer weights
- Fix `RangeError` when `l.sample_values(count, { replace: false })` has `count` > `l.length`


## v0.1.2

### `FrozenWeightedList`
- Enforce stronger immutability with `Object.freeze` and `Readonly<>`
- Use `int` where appropriate in interface


## v0.1.1

### `FrozenWeightedList`
- Fix edge case errors in `.at()` indexing


## v0.1.0

- Initial release on npm!
