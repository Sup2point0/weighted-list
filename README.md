<h1 align="center"> <code> weighted-list </code> </h1>

<div align="center">
  <img alt="Rust Tests Status"
    src="https://github.com/Sup2point0/weighted-list/actions/workflows/test-rs.yml/badge.svg" />
  <img alt="Python Tests Status"
    src="https://github.com/Sup2point0/weighted-list/actions/workflows/test-py.yml/badge.svg" />
  <img alt="C# Tests Status"
    src="https://github.com/Sup2point0/weighted-list/actions/workflows/test-cs.yml/badge.svg" />
  <img alt="TypeScript Tests Status"
    src="https://github.com/Sup2point0/weighted-list/actions/workflows/test-ts.yml/badge.svg" />
  <img alt="Haskell Tests Status"
    src="https://github.com/Sup2point0/weighted-list/actions/workflows/test-hs.yml/badge.svg" />
  <!-- <img alt="Ruby Tests Status"
    src="https://github.com/Sup2point0/weighted-list/actions/workflows/test-rb.yml/badge.svg" /> -->

  [walkthrough](walkthrough.md) · [rationale](rationale.md) · [spec](SPEC.md)

</div>

A list implementation for weighted randomisation, implemented (eventually) in every programming language I’ve learnt.


<details open>
  <summary>
    <strong> Rust </strong>
  </summary>

```rs
let descriptors = wlist![
    (10, "cool".to_string()),
    (5,  "awesome".to_string()),
    (2,  "elegant".to_string()),
    (1,  "beautiful".to_string()),
];

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

</details>

<details>
  <summary>
    <strong> Python </strong>
  </summary>

```py
greetings = WeightedList((20, "sup"), (2, "salutations"))

print(greetings.select())
# => sup
```

</details>

<details>
  <summary>
    <strong> C# </strong>
  </summary>

```cs
WeightedList<string, int> greetings = new((20, "sup"), (2, "salutations"));

Console.WriteLine(greetings.GetRandomValue());
// => salutations
```

</details>

<details>
  <summary>
    <strong> TypeScript </strong> (under development)
  </summary>

```ts
let greetings = new WeightedList([20, "sup"], [2, "salutations"]);

console.log(greetings.select_value());
// => sup
```

</details>

<details>
  <summary>
    <strong> Haskell </strong> (under development)
  </summary>

```hs
greetings :: WeightedList String Int
greetings = newWeightedList [(20, "sup"), (2, "salutations")]

main :: IO ()
main = print (selectValue greetings)
-- => salutations
```

</details>

<details>
  <summary>
    <strong> Ruby </strong> (awaiting development)
  </summary>

```rb
# Ruby (working on it!)
```

</details>

An immutable optimised variant `FrozenWeightedList` is also implemented, which provides $O(\log{n})$ item access.


<br>


## Features

- Weighted randomised selection with constraints such as no-replacement, unique-only
- Ergonomic interface targeted for each language
- Utility methods to manipulate values and weights
- In-place and pure variants of methods for flexibility
- Conversions to and from other data types

### Future
- Slice indexing[^slice]

[^slice]: Really quite difficult with non-integer weights.


<br>


## Purposes

> [!Tip]
> For the full rationale behind this project, see [rationale](rationale.md).

I made this class for *weighted randomisation*, where each element in a collection has a different chance of being selected – the greater an item’s weight, the higher the chance it is selected. This is super common in games for reward systems, displaying messages, etc.


<br>


## Usage

> [!Tip]
> Walkthroughs and specimens for each language can be found in their respective folders.

### Rust
`weighted-list` is now on [crates.io](https://crates.io/crates/weighted-list)! Add the crate to your Rust project by running:

```bash
> cargo add weighted-list
```

For more info on how to import and use the crate, see [docs.rs](https://docs.rs/crate/weighted-list/latest).

### Python
All you need is the [`weightedlist.py`](python/source/weighted_list.py) file, which contains the `WeightedList` class with all the functionality. Simply import it, and you’re ready to go!

```py
from weightedlist import WeightedList
```

See [walkthrough](python/walkthrough.md) for a tutorial, or [examples](python/examples.md) for examples.

### C#
All the code is contained within the [`WeightedList.cs`](c-sharp/weighted-list/weighted-list.cs) file. You might also need the [`weighted-list.csproj`](c-sharp/weighted-list/weighted-list.csproj) file. If you want the entire solution, you can download the repo and extract the [`c-sharp/`](c-sharp/) folder.

For a tutorial, see [walkthrough](c-sharp/walkthrough.md).


<br>


## Compatibility

| Language   | Version   | Status | Dependencies | Notes |
| :--------- | :-------- | :----- | :----------- | :---- |
| Rust       | `2024`    | Unstable | `rand`, `num_traits`, `itertools`, `bon` |
| Python     | `>= 3.11` | Awaiting rewrite | None |
| C#         | `12.0`    | Awaiting maintenance | None | Supports LINQ querying |
| TypeScript |           | Under development | None |
| Haskell    | `GHC2021` | Under development | None |
| Ruby       |           | Awaiting development |


<br>


## Implementation

A `WeightedList` works similar to a regular list/array, except rather than storing raw values, it stores them inside `WeightedItem` objects. The value and weight of each item can be accessed through the `value` and `weight` attributes, respectively.

`WeightedList` using *weighted* indexing. To illustrate with an example:

```py
wl = WeightedList((1, "qi"), (2, "sup"), (5, "shard"))

assert wl[0].value == "qi"
assert wl[1].value == "sup"
assert wl[2].value == "sup"
assert wl[3].value == "shard"
assert wl[4].value == "shard"
assert wl[5].value == "shard"
assert wl[6].value == "shard"
assert wl[7].value == "shard"

wl[8]  # IndexError
```

In essence, you can think of each item as being repeated a number of times, equal to its weight.


<br>


## Questions

### Why did you create this?
Back when I was picking up the ropes of Python, I was working on a project which featured randomisation, and, like any game developer, I thought it’d be cool to give each outcomes different probabilities of occurring. At first, I achieved this behaviour by duplicating items, but I quickly realised the numerous issues with this.

And so, I set out to write my own class, which I’d never really needed to do up until that point. I thought it’d be a great exercise in learning Python – and it very much was, teaching me tons about object-oriented programming, dunder methods, generators, etc. It was also my first experience of conscientiously writing code that wasn’t exclusively for myself, which helped me understand the importance of consistency and clarity, and above all, documentation.

A couple years later, I’ve come back to do the same in C#, this time also adding several features I always intended to add but never did – especially non-integer weights, which allows the class to truly embrace its usage as representing probabilities. Trying to translate Python into C# was an interesting experience,[^translate] and helped highlight some important differences between the languages that I would otherwise not have found out.

[^translate]: This was not exactly the way I created the project in C#, but the Python implementation certainly laid out a general framework and was influential in some design decisions.

A few more years later, I’m back to do the same in Haskell and Rust (and also finish off the TypeScript and Ruby implementations that I started but never finished). Damn I love this project. Seriously, it never fails to raise so many questions about a language’s mechanics and quirks that I would never encounter otherwise.

### Is this even useful?
I mean yeah, a whole several-hundred-lines class to handle one thing is probably overkill... it’s more an exercise and proof-of-concept.

Regardless, I’ve used my own code[^surprise] in at least 2 major projects ([PENGUIN<sup>↗</sup>](https://github.com/Sup2point0/PENGUIN) and [Algorhythm<sup>↗</sup>](https://Sup2point0/Algorhythm)), so I can definitely say it’s been useful to me!

[^surprise]: To my own surprise, somewhat.

### Why are the source files several hundred lines long?
1. documentation
2. line breaks
3. utility

Particularly documentation. That stuff just *eats* the line count. Also, implementing something as complex as an enumerable container requires a lot of methods, operators, interfaces and delegation. And in C# you've even got overloading to account for as well.

### How fast is it?
In all honesty, I don’t know. I’m slowly adding benchmarks to test different approaches.

### Why is your Python code not compliant to PEP 8?
I have my own particular preferences when it comes to coding in Python, which I explain fully [here<sup>↗</sup>](https://github.com/Sup2point0/Assort/blob/origin/~dev/Python%20Syntax.md).

### Why do you start `{}` on a new line?
I’m a C# programmer, what can I say :P

### Why do you use `snake_case` in TypeScript?
I’m a Python programmer, what can I say :P

### Why are you okay with `camelCase` in Haskell then?
I used `snake_case` before, and ngl, in Haskell you kinda need the `camelCase` to keep things readable without parentheses...


<br>


## Contribute

Any feedback, suggestions or improvements are definitely welcome!


<br>


<!-- what you lookin at? -->
