# Walkthrough

> This acts as a guided overview of how to utilize `WeightedList`s.

## Instantiation
Creating a `WeightedList` is similar to creating a list, except each item should be a weight-value pair, such as a `tuple`:

```py
wl = WeightedList(
  (2, "sup"),
  (7, "nova"),
  (13, "shard"),
  ...
)
```

Note that the weight always comes first, so that it consistently aligns. [^0]

Keyword arguments can also be used, where each key and value correspond to an item value and weight, respectively:

```py
wl = WeightedList(sup = 2, nova = 7)
```

However, note that this only allows `str` values with no spaces or duplicates.

Many methods involve ‘compatible iterables’. This refers to any sequence or mapping that can be unpacked during instantiation to provide arguments, shown above.

For sequences, they must consist of item pairs where the weight comes first. For mappings, keys and values correspond to item values and weights, respectively.

## Indexing
The items in a `WeightedList` are stored as `WeightedItem` objects, which have a `value` and `weight` attribute. When indexing the list, these objects are returned:

```py
>>> wl[0]
WeightedItem('sup', 2)
>>> wl[0].value
'sup'
>>> wl[0].weight
2
```

A `WeightedList` uses *weighted indexing*, which drives the weighting machanic. The `weight` of each item can be thought of as how many times that object is repeated.

```py
>>> wl[0].value
'sup'
>>> wl[1].value
'sup'
>>> wl[2].value
'nova'
>>> wl[8].value
'nova'
```

Calling `len()` on it, notice that the length takes weights into account:

```py
>>> len(wl)
9
# 'sup' has a weight of 2
# 'nova' has a weight of 7
# the length (total weight) is 9
```

## Selection
The most important method, that the whole class essentially revolves around, is `self.select()`. This randomly selects an item from the list, taking weights into account. Items with higher weight have a higher chance of being selected, and vice versa for lower weights.

The simplest way is calling it with no arguments:

```py
>>> wl.select()
'nova'
```

This selects a single item, and returns its value. To obtain the item itself (which would allow its weight to be accessed), set `depth` to `True`:

```py
>>> wl.select(depth = True)
WeightedItem('sup', 2)
```

Multiple items can be selected at once, by passing in an `int`:

```py
>>> wl.select(2)
['nova', 'nova']
```

2 `int`s can be passed, to select a random number of items between those 2 numbers (inclusive):

```py
# selects anywhere between 1 to 10 items
>>> wl.select(1, 10)
[...]
```

This selects items *with replacement*, which means the ‘same’ item can be selected more than once. To select without replacement, set `replace` to `False`:

```py
>>> wl.select(20, replace = False)
['nova', 'nova', 'sup', 'nova', 'nova', 'nova', 'nova', 'sup', 'nova']
```

## Modification
All the regular methods a `list` can be used on a `WeightedList`; however, some work slightly differently.

All return the *modified* list afterwards, to allow for chaining should you wish to do so:

```py
>>> wl = (WeightedList(...)
.clear()
.append(2, "sup")
.append(7, "nova")
.shuffle()
.cluster(3)
.deviate()
.increment(-2)
.extrapolate()
)
```

`append()` takes 2 arguments, a `value` and `weight`. Again, the weight comes first, for alignment:

```py
>>> wl.append(13, "shard")
WeightedList(('sup', 2), ('nova', 7), ('shard', 13))
```

If no weight is passed, it defaults to `1`:

```py
>>> wl.append("sip")
WeightedList(('sup', 2), ('nova', 7), ('shard', 13), ('sip', 1))
```

`extend()` can take multiple iterables as arguments. These can be `WeightedList`s or any [compatible iterables](Indexing).

## Manipulation
Weights can be manipulated through various methods to distort or change weightings.

`increment` changes all weights by a particular amount, which can be negative.

```py
>>> wl.increment()
WeightedList(('sup', 3), ('nova', 8))
>>> wl.increment(2)
WeightedList(('sup', 5), ('nova', 10))
>>> wl.increment(-4)
WeightedList(('sup', 1), ('nova', 6))
```

Increasing all weights has the effect of lessening the relative difference between them, and vice versa for decreasing.

## Conversion
It may be useful to convert a `WeightedList` to another data type for transfer or storage.

The values and weights can be accessed through those respective properties:

```py
>>> wl.values
['sup', 'nova']
>>> wl.weights
[2, 7]
```

`list` and `dict` representations can be extracted too:

```py
>>> wl.list
[[2, 'sup'], [7, 'nova']]
>>> wl.dict
{'sup': 2, 'nova': 7}
```