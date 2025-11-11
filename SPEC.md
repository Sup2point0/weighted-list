# Specification

> Quicklink: [Jump to](#fields) fields reference

I don’t know how to write a specification. (Yet.) But this is just so I can keep track of what I need to tick off when implementing `WeightedList` in a particular programming language.


<br>


## Terminology

> [!Note]
> Most of this spec will use `WeightedList` to simultaneously refer to both `WeightedList` and `FrozenWeightedList` for brevity, since the latter is essentially a subset of the former.

| Term | Definition |
| :--- | :--------- |
| *implementation language* | The programming language in which the code is written. |
| *list* | In the context of this project, this refers to a `WeightedList` or `FrozenWeightedList`. |
| *compatible iterable* | An iterable type that can be converted to a `WeightedList`. This can be: <ul> <li>A sequence of (weight, value) 2-value iterables (e.g. `list[(Weight, Value)]`)</li> <li>A mapping between (unique) values and weights (e.g. `dict[Value, Weight]`)</li> </ul> |


<br>


## General

- Implement 2 classes:
  - `WeightedList` (mutable)
  - `FrozenWeightedList` (immutable, optimised)
- Where applicable, these classes should derive from an appropriate built-in iterable type in the implementation language.
  - For `WeightedList`, this should be a variable-length collectinon type.
  - For `FrozenWeightedList`, this may be an optimised fixed-length collection type.

### `WeightedList`
- A weighted list is an ordered collection of weighted items.
- Each item has a **weight** and a **value**.
  - The weight must be a **positive numerical** type (`Weight`).
    - Items with non-positive (i.e. negative or `0`) weights may be removed.
  - The value can be any type (`Value`).
    - Values do not have to be unique between items.
- When initialising weighted items, the weight always comes before the value.
  - This ensures tidier layout when enumerating many items.
- Indexing is based on item weights, as opposed to the position of items relative to each other.

### `FrozenWeightedList`
- A ***frozen*** weighted list is an immutable variant of the weighted list.
- It does not have any methods that mutate the list, but shares all other methods used for accessing/querying the items.
- Item access should be optimised to $O(\log{n})$ time complexity.


<br>


## Implementation

- `WeightedList` implements all members it inherits from the built-in array-like class of the implementation language.
- Methods are specialised to handle weighted items.
  - Where applicable, raw values should be handled too.
    - When an item is provided with no weight, its weight defaults to 1.
- Where applicable and feasible, both in-place and out-of-place variants of methods are implemented.
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


<br>


## Fields

> [!Note]
> Types and code samples here use a Python-like syntax to demonstrate details across all implementation languages.

- Core fields that are guaranteed to be implemented in all languages are **weighted**.[^weighted]
  - Language-dependent fields are *italicised*. They may be implemented with multiple individual variants depending on overloading capability.
- *Time Complexity* indicates the target time complexity of an implementation. Performance may vary across languages.
- By default, if a method has no need to return a value, it will return the modified `WeightedList` (indicated below as *default*).

[^weighted]: Hah, pun intended.

### Accessors
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| <table> <tr><td><strong>length</strong></td></tr> <tr><td>total weights</td></tr> </table> | Total weights of all items. | – | `Weight` | – | $O(n)$ |
| total items | Total number of items. | – | `int` | – | $O(n)$ |
| **get values** | Get the values of all items. | – | `iter[Value]` | – | lazy | Values are in order. |
| **get weights** | Get the weights of all items. | – | `iter[Weight]` | – | lazy | Weights are in order. |
| **get raw** | Get the (weight, value) representations of all items. | – | `iter[(Weight, Value)]` | – | lazy | This usually satisifes the axiom that for any list `wl` we have `WeightedList(wl.raw()) == wl` |

### List Methods
These are usually inherited from the built-in array type of the language and adapted for a `WeightedList`/`FrozenWeightedList`.

#### Non-Mutating
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **get item** | Get an item. | The weighted index $i$ | `WeightedItem` | – | $O(i)$ |
| *find item* | Find an item(s) that fulfil a predicate. | May include: <ul> <li>Item to compare equality against</li> <li>Predicate to match against</li> </ul> | `WeightedItem` or <br> `iter[WeightedItem]` | – | $O(n)$ | Multiple variations may be implemented for finding 1 or many items, matching against equality or predicate, etc. |
| *find index of item* | Find the (weighted) index of item(s) that fulfil a predicate. | May include: <ul> <li>Item to compare equality against</li> <li>Predicate to match against</li> <li>Whether to return a weighted or unweighted index</li> </ul> | `Weight` or <br> `iter[Weight]` or <br> `int` or <br> `iter[int]` | – | $O(n)$ | Multiple variations may be implemented for finding 1 or many items, matching against equality or predicate, etc. |

#### Mutating
| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| **append item** | Append an item. | The `WeightedItem` to append | default | yes | $O(1)$ | May require additional memory allocation. |
| *append item* | Append an item with the given *value* and a weight of $1$. | The `Value` of the item to append | default | yes | $O(1)$ | Implementations may merge this into *append item* using overloading or type checks. |
| **insert item** | Insert an item at a given (weighted) index $i$. | The `WeightedItem` to insert | default | yes | $O(i)$ | May require additional memory allocation. |
| *insert value* | Insert an item with the given *value* and a weight of $1$ at a given (weighted) index $i$. | The `Value` of the item to insert | default | yes | $O(i)$ | Implementations may merge this into *insert item* using overloading or type checks. |
| **remove at index** | Remove an entire item. | The weighted index $i$ | the removed `WeightedItem` | yes | $O(i)$ |
| **drop at index** | Decrement the weight of an item. | <table> <tr> <th>weighted-index</th> <td>The weighted index $i$</td> </tr> <tr> <th>by</th> <td>The amount by which to decrement the item’s weight.</td> </tr> </table> | the modified `WeightedItem` | yes | $O(i)$ | If the item’s weight becomes $0$ or negative as a result, it is removed. |

### WeightedList Methods
These are special for `WeightedList`?`FrozenWeightedList`.

| Field | Description | Options | Returns | In-Place | Time Complexity | Notes |
| :---- | :---------- | :------ | :------ | :------- | :-------------- | :---- |
| select random value | Randomly select $1$ *value* from the list. | – | `Value` | – | $O(n)$ |
| **select random item** | Randomly select $1$ item from the list. | – | `WeightedItem` | – | $O(n)$ |
| **select random values** | Randomly select $k$ values from the list. | <ul> <li>The number of items to select</li> <li>Whether to only output unique values</li> <li>Whether to select with replacement</li> </ul> | `iter[Value]` | – | $O(kn$) |
| **pop random item** | Randomly select and drop $1$ item from the list. | <table> <tr> <th>drop?</th> <td>How much to decrease the item’s weight by, or whether to remove the item entirely.</td> </tr> </table> | `Item` | yes | $O(n)$ | <ul> <li>In-place version of *select random item*.</li> <li>For compatible languages, *drop* may be a `Weight` or `Boolean`. If 2 types is not possible, this parameter may be split into 2 individual parameters.</li> </ul> |
| **pop random values** | Randomly select and drop $k$ values from the list. | <table> <tr> <th><em>k</em></th> <td>Same as above.</td> <tr> <th>unique?</th> <td>Same as above.</td> <tr> <th>replace?</th> <td>Same as above.</td> </tr> </tr> </tr> <tr> <th>drop?</th> <td>Same as above.</td> </tr> </table> | `Item` | yes | $O(n)$ | <ul> <li>In-place version of *select random values*.</li> </ul> |
| <!-- $m$ refers to length of other list -->
| **prune** | Remove items with invalid (non-positive) weights | – | default | both | $O(n)$ |
| collapse | Merge duplicates items by summing their weights. | – | default | both | $O(n)$ |
| normalise | Normalise weights such that total weight becomes $1.0$. | – | default | both | $O(n)$ |
| shuffle | Shuffle (weight, value) pairings. | – | default | both | $O(n)$ | Shuffling is performed on weights, such that values remain in their original order. |
| **merge** | Merge list with another compatible iterable. | – | default | both | $O(mn)$ | For each item in the other iterable, if it exists in the current list, the weight of the first equal item is increased accordingly; otherwise, the item is appended to the current list. |


<br>
