# Specification

> v1.1.0  
> Last updated: 11 February 2025

> [!Tip]
> Quicklink: [Jump to fields reference](#fields)

I don’t know how to write a specification. (Yet.) But this is just so I can keep track of what I need to tick off when implementing `WeightedList` in a particular programming language.


<br>


## Terminology

> [!Note]
> Most of this spec will use `WeightedList` to simultaneously refer to both `WeightedList` and `FrozenWeightedList` for brevity, since the latter is essentially a subset of the former.

| Term | Definition |
| :--- | :--------- |
| *implementation language* | The programming language in which the code is written. |
| *list* | In the context of this project, this refers to a `WeightedList` or `FrozenWeightedList`. |
| *item* | A `WeightedItem` or `FrozenWeightedItem` inside a list. |
| *compatible iterable* | An iterable type that can be converted to a `WeightedList`. This can be: <ul> <li>A sequence of (weight, value) 2-value iterables (e.g. `list[(Weight, Value)]`)</li> <li>A mapping between (unique) values and weights (e.g. `dict[Value, Weight]`)</li> </ul> |


<br>


## General

- Implement 2 classes:
  - `WeightedList` (mutable)
  - `FrozenWeightedList` (immutable, optimised)
- Where applicable, these classes should derive from an appropriate built-in iterable type in the implementation language.
  - For `WeightedList`, this should be a variable-length collection type.
  - For `FrozenWeightedList`, this may be an optimised fixed-length collection type.
- Where appropriate, these classes should be generic with type parameters for values and (optionally) weights.

### `WeightedList`
- A weighted list is an ordered collection of weighted items.
- Each item has a **weight** and a **value**.
  - The weight must be a **positive numerical** type (`Weight`).
    - Behaviour for items with negative weight is undefined.
  - The value can be any type (`Value`).
    - Values do not have to be unique between items.
- When initialising weighted items, the weight always comes before the value.
  - This ensures tidier layout when enumerating large numbers of items.

### `FrozenWeightedList`
- A ***frozen*** weighted list is an immutable variant of a weighted list.
- It does not have any methods that mutate the list, but shares all other methods used for accessing/querying the items.
- Item access should be optimised to $O(\log{n})$ time complexity.


<br>


## Implementation

- `WeightedList` implements all members it inherits from the built-in array-like class of the implementation language.
- Methods are specialised to handle weighted items.
  - Where applicable, raw values should be handled too.
    - When an item is provided with no weight, its weight defaults to 1.
- Where applicable and possible, both in-place and out-of-place variants of methods are implemented.
  - If the implementation language has no convenient way to indicate whether a method is in-place or out-of-place, this is achieved through the naming of the method.
    - In-place methods are **simple present** verbs (“merge”, “prune”).
    - Out-of-place methods are **present perfect** participles (“merged”, “pruned”).
  - Out-of-place methods create a shallow copy of the list to prevent modifying the original list.
    - The returned list still contains references to the original items.


<br>


## Interfaces

### Iterable
- `WeightedList` implements the appropriate iterable/iterator protocol for the implementation language.
  - This ultimately should mean iteration with a loop or map over the list is possible:

```py
for item in WeightedList(...):
    print(item)
```

### Equality

> [!Important]
> The following equality checks use *value* equality, not *reference* equality. For instance, this refers to `==` as opposed to `is` in Python.

- 2 `WeightedItem`s are equal if:
  - Their values are equal
  - Their weights are equal
- 2 `WeightedList`s are equal if:
  - They contain the same number of items
  - Each pair of items between the lists in order is equal

### Indexing
- The list should be indexable using the implementation language’s standard indexing notation (`list[index]` for most).
- Indexing uses *weighted indexing*, which considers item weights rather than their positions relative to each other.


<br>


## Fields

> [!Note]
> Types and code samples here use a Python-like syntax to demonstrate details across all implementation languages.

- Core fields that are guaranteed to be implemented in all languages are **weighted**.[^weighted]
  - Language-dependent fields are *italicised*. They may be implemented with multiple individual variants depending on overloading capability.
- *Time Complexity* indicates the target time complexity of an implementation. Performance may vary across languages.
  - $n$ refers to the number of items in the current list.
  - $m$ refers to the number of items in another iterable.
- By default, if a method has no need to return a value, it will return the modified `WeightedList` (indicated below as *default*).

[^weighted]: Hah, pun intended.

### Properties
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **length**           | Total weights of all items. | – | `Weight` | – | $O(n)$ |
| total weight         | Total weights of all items (alias for `.length`) | – | `Weight` | – | $O(n)$ |
| **total values**     | Total number of values/items. | – | `int` | – | $O(n)$ |
| is zero              | Do all items (if any) have a weight of zero? | – | `bool` | – | $O(n)$ | Returns `true` for an empty list. |
| has negative weights | Do any items have a negative weight? | – | `bool` | – | $O(n)$ | Returns `false` for an empty list. |

### Accessors
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **weights**      | Iterate over the weights of all items. | – | `iter[Weight]` | – | lazy | Weights are in order. |
| **values**       | Iterate over the values of all items. | – | `iter[Value]` | – | lazy | Values are in order. |
| **raw**          | Iterate over the `(weight, value)` representations of all items. | – | `iter[(Weight, Value)]` | – | lazy | This usually satisifes the axiom that for any list `wl` we have `WeightedList(wl.raw()) == wl`. |
| expanded         | Iterate over the values of all items, each value duplicated a number of times equal to its weight. |
| collect weights  | Get the weights of all items. | – | `list[Weight]` | – | $O(n)$ |
| collect values   | Get the values of all items. | – | `list[Value]` | – | $O(n)$ |
| collect raw      | Get the `(weight, value)` representations of all items. | – | `list[(Weight, Value)]` | – | $O(n)$ |

### List Methods
These are usually inherited from the built-in array type of the language and adapted for a `WeightedList`/`FrozenWeightedList`.

#### Non-Mutating
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **get item** | Get an item at the specified weighted index. | The weighted index | `WeightedItem` | – | $O(n)$ |
| *find item* | Find an item(s) that fulfils a predicate. | May include: <ul> <li>Item to compare equality against</li> <li>Predicate to match against</li> </ul> | `WeightedItem` or <br> `iter[WeightedItem]` | – | $O(n)$ | Multiple variations may be implemented for finding 1 or many items, matching against equality or predicate, etc. |
| *find index of item* | Find the (weighted) index of item(s) that fulfil a predicate. | May include: <ul> <li>Item to compare equality against</li> <li>Predicate to match against</li> <li>Whether to return a weighted or unweighted index</li> </ul> | `Weight` or <br> `iter[Weight]` or <br> `int` or <br> `iter[int]` | – | $O(n)$ | Multiple variations may be implemented for finding 1 or many items, matching against equality or predicate, etc. |

#### Mutating
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **append item**     | Add an item to the end of the list. | The `WeightedItem` to append | default | yes | $O(1)$ | May require additional memory allocation. |
| *append value*      | Append an item with the given *value* and a weight of $1$. | The `Value` of the item to append | default | yes | $O(1)$ | Implementations may merge this into *append item* using overloading or type checks. |
| **insert item**     | Insert an item at a given weighted index. | The `WeightedItem` to insert | default | yes | $O(n)$ | May require additional memory allocation. |
| *insert value*      | Insert an item with the given *value* and a weight of $1$ at a given (weighted) index. | The `Value` of the item to insert | default | yes | $O(n)$ | Implementations may merge this into *insert item* using overloading or type checks. |
| **remove at index** | Remove an entire item. | The weighted index | the removed `WeightedItem` | yes | $O(n)$ |
| **take at index**   | Decrement the weight of an item. | <ul> <li>The weighted index</li> <li>How much to decrement the weight by</li> </ul> | the modified `WeightedItem` | yes | $O(n)$ | If the item’s weight becomes $0$ or negative as a result, it is removed. |

### WeightedList Methods
These are special for `WeightedList`/`FrozenWeightedList`.

#### Mutating
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| merge item       | Merge an item into the list. | – | default | both | $O(n)$ | <ul> <li>If an item already exists in the list with the same value, its weight is increased by the incoming item’s weight; otherwise, the incoming item is appended to the list.</li> </ul> |
| merge with       | Merge the list with another compatible iterable. | – | default | both | $O(mn)$ |
| merge duplicates | Merge duplicate items in the list. | – | default | both | $O(n^2)$ | <ul> <li>The duplicate items are merged into a single item with their combined weights.</li> <li>The new item will be in the same position as the original first occurrence of a duplicate.</li> </ul> |

#### Random Selection
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **select random item**   | Randomly select $1$ item from the list. | – | `Item` | – | $O(n)$ |
| select random value      | Randomly select $1$ *value* from the list. | – | `Value` | – | $O(n)$ |
| **select random values** | Randomly select $k$ values from the list. | <ul> <li>How many values to select</li> <li>Whether to select with replacement</li> </ul> | `iter[Value]` | – | $O(kn$) |
| **select random values (unique)** | Randomly select $k$ *unique* values from the list. | <ul> <li>How many values to select</li> <li>Whether to treat items with the same value as non-unique</li> </ul> | `iter[Value]` | – | $O(kn)$ | By default, 2 items with the same value are treated as unique (so both could be picked, but each one only once). |
| **take random item**   | Randomly take $1$ item from the list. | How much to decrease the item’s weight by, or whether to remove the item entirely | `Item` | yes | $O(n)$ | <ul> <li>For compatible languages, *drop* may be a `Weight` or `Boolean`. If 2 types is not possible, this parameter may be split into 2 individual parameters.</li> <li>In-place version of *select random item*.</li> </ul> |
| **take random values** | Randomly take $k$ values from the list. | <ul> <li>How many values to select</li> <li>How much to decrease each item’s weight by, or whether to remove the item entirely</li> </ul> | `iter[Item]` | yes | $O(kn)$ | In-place version of *select random values*. |
| **take random values (unique)** | Randomly take $k$ *unique* values from the list. | Whether to treat items with the same value as non-unique | `iter[Value]` | yes | $O(kn)$ | In-place version of *select random values (unique)*. |

#### Weight Manipulation
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| normalise    | Normalise weights such that total weight becomes $1.0$. | – | default | both | $O(n)$ |
| **prune**    | Remove items with invalid (non-positive) weights | – | default | both | $O(n)$ |
| set weights  | Set the weights of all items in the list to a given weight. | The new weight | default | both | $O(n)$ |
| shuffle      | Shuffle `(weight, value)` pairings. | – | default | both | $O(n)$ | Shuffling is performed on weights, such that values remain in their original order. |
| zero weights | Set the weights of all items in the list to $0$$. | – | default | both | $O(n)$ |


## Rationale

### Why are weights of $0$ allowed?
Processes may emit a warning if they encounter an item with a weight of $0$, but this is not a fatal issue since the presence of these items don’t really affect anything. It’s also possible the user may wish to, for instance, initialise items with weights of $0$ and increment them in some way.


## Implementation Checklist

> [!Note]
> This is for personal use when implementing `WeightedList` in a new language.

<!-- # Implementation Checklist
v1.0.1 -->

### `WeightedItem`
- [ ] print
- [ ] eq

### `WeightedList`
- Constructors
  - [ ] [(weight, value)]
- Interfaces
  - [ ] print
  - [ ] iter/map
  - [ ] eq
- Accessors
  - [ ] total weight
  - [ ] total items
  - [ ] get weights
  - [ ] get values
  - [ ] get raw
- List Methods
  - [ ] get item
  - [ ] find item
  - [ ] find index of item
  - [ ] append item
  - [ ] append value
  - [ ] insert item
  - [ ] insert value
  - [ ] remove at index
  - [ ] pop at index
- WeightedList Methods
  - [ ] select random value
  - [ ] select random item
  - [ ] select random values
  - [ ] pop random item
  - [ ] pop random values
  - [ ] prune
  - [ ] collapse
  - [ ] normalise
  - [ ] shuffle
  - [ ] merge


<br>
