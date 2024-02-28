# Walkthrough / General

This provides a general overview of the underlying workings of a `WeightedList`, which remains the same across both Python and C#. For language-specific walkthroughs, visit their respective folders.


<br>


## Overview

A `WeightedList`, as its name should indicate, is an extension of a regular list and is intended to be used in a similar way. As such, it inherits many properties and methods from a list, so if you’re familiar with lists then a weighted list shouldn’t be too difficult to pick up.

The key difference is that, instead of containing the values directly like a list, a `WeightedList` stores them as `WeightedItem` objects with value and weight attributes. This is a very lightweight class which only serves to contain these 2 attributes, so it can be helpful to think of a `WeightedItem` as simply a 2-item tuple (that is mutable). Manipulating the contents of the `WeightedList` revolves around handling `WeightedItem`s.


<br>


## Creation

`WeightedList` offers several ways to pass in its data for different circumstances.

### Tuples
The cleanest way[^clean] is by passing the data as tuples.

[^clean]: This is, of course, only an opinion. Use whichever way you prefer or is most suitable for the situation.

<details>
  <summary> <b> Python </b> </summary>

```py
wl = WeightedList(
  (2, "sup"),
  (3, "nova"),
)
```

</details>

<details>
  <summary> <b> C# </b> </summary>

```cs
WeightedList<string, int> wl = new(
    (2, "sup"),
    (3, "nova")
);
```

</details>

> [!IMPORTANT]
> The weight comes **before** the value *only for tuples*. This allows the weights to align nicely so it’s clear which value they correspond to. It may seem counter-intuitive at first, but comes in immensely helpful when handling long weighted lists holding values all with varying lengths.

### Weighted Items
Both the Python and C# implementations provide the option to use `WeightedItem`s, though this is not recommended since it is significantly more verbose than any other method.

<details>
  <summary> <b> Python </b> </summary>

```py
wl = WeightedList(
  WeightedList.WeightedItem("sup", 2),
  WeightedList.WeightedItem("nova", 3),
)
```

</details>

<details>
  <summary> <b> C# </b> </summary>

```cs
WeightedList<string, int> wl = new(
    new WeightedItem<string, int>(2, "sup"),
    new WeightedItem<string, int>(3, "nova")
);
```

</details>

> While you would never want to write this code explicitly, support for passing in data as `WeightedItem`s is still entirely necessary, since it allows data from *other* `WeightedList`s to be used.

### Dictionary
If passing in data as a dictionary, each key-value pair represents a `WeightedItem`, with the key and value corresponding to the item’s value and weight respectively.

<details>
  <summary> <b> Python </b> </summary>

```py
wl = WeightedList({
  "sup": 2,
  "nova": 3,
})
```

</details>

<details>
  <summary> <b> C# </b> </summary>

```cs
Dictionary<string, int> data = new {
    ["sup"] = 2,
    ["nova"] = 3
};

WeightedList<string, int> wl = new(data);
```

</details>


<br>


## Indexing

> [!TIP]
> If you haven’t read the [rationale](../rationale.md), now would be a good time to do so, since it illustrates the underlying concept behind the following implementation.

Perhaps the most confusing aspect of a `WeightedList` is how its indexing works. Unlike a regular list, a `WeightedList` uses *weighted indexing*.[^wix] Essentially, imagine each value is repeated a number of times in the list equal to its weight. So for a `WeightedList` containing `"sup"` with a weight of `2` and `"nova"` with a weight of `3`, its indices would work like so:

[^wix]: Note that this is an unofficial term coined by me.

<table>
  <tr>
    <th> index </th>
    <td align="center"> 0 </td>
    <td align="center"> 1 </td>
    <td align="center"> 2 </td>
    <td align="center"> 3 </td>
    <td align="center"> 4 </td>
  </tr>
  <tr>
    <th> value </th>
    <td> sup </td>
    <td> sup </td>
    <td> nova </td>
    <td> nova </td>
    <td> nova </td>
  </tr>
</table>

This weighted indexing is what allows the weighted randomisation to occur. Notice this also means it is unclear what the length of the `WeightedList` should be defined as – so instead, it has a ‘total values’ property to count the number of distinct items, and a ‘total weights’ property to count the total weight of all the items.


<br>


## Final Notes

While the Python and C# implementations are very similar, they have some significant differences.
- The Python `WeightedList` is a direct subclass of `list`, and either inherits or overrides all of its attributes and methods. The C# `WeightedList<>` implements most of the methods and interfaces of `List<>`, but is not a subclass.
- The Python `WeightedList` *is* the data, so methods manipulate the data via `self.method()`. The C# `WeightedList<>` *contains* the data in a private `_data` field, so methods manipulate the data via `_data.Method()`. Externally, this makes no difference to how the 2 versions function.
- The Python `WeightedItem` is an internal class of `WeightedList`, so is accessed via `WeightedList.WeightedItem`. The C# `WeightedItem<>` is a standalone class defined in the same file as the `WeightedList<>` class.
- The C# `WeightedList<>` is strongly typed, so can only hold values of a single type (unless boxing them as `object`s), but also supports non-integer weights.


<br>
