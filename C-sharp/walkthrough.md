# Walkthrough / C#

This article guides you through how to use the `WeightedList` class in C#.

> [!Note]
> While all the code written here shows how the class was intended to be used, how you use it is entirely up to you – use whichever style you feel most comfortable with!


<br>


## Overview

In C#, `WeightedList` is implemented as a generic class `WeightedList<V, W>`. `V` represents the types of values the list will store, and as such can be any type. `W` represents the types of weights the items will have, and must be a number-like type (specifically, one that implements `INumber`).

The list stores items as generic `WeightedItem<V, W>` objects with the same type parameters. Each item has `Value` and `Weight` fields storing its information.

> [!Tip]
> It’s not a bad idea to take a peek at the source code, at least the top parts of the `WeightedItem` and `WeightedList` classes, to understand how they work.


<br>


## Constructors

`WeightedList` provides numerous ways to pass in data for flexibility and versatility. To cast a different container to a `WeightedList`, just use the constructor and pass it in instead.

The simplest way is through a parameter list of tuples:

```cs
WeightedList<string, int> wl = new(
    (2, "sup"),
    (3, "nova")
);
```

> [!Note]
> When passing data as tuples, the weight always comes first. This allows the weights to align nicely so it’s clear which value they correspond to, which is helpful with especially long lists holding values with varying lengths.

The same thing can be achieved with any `IEnumerable` of tuples:

```cs
List<string, int> data = [
    (2, "sup"),
    (3, "nova")
];

WeightedList<string, int> wl = new(data);
```

The constructor also accepts `WeightedItem`s instead of tuples:

```cs
WeightedList<string, int> wl = new(
    new WeightedItem<string, int>(2, "sup"),
    new WeightedItem<string, int>(3, "nova")
);
```

```cs
List<WeightedItem<string, int>> data = [
    new WeightedItem<string, int>("sup", 2);
    new WeightedItem<string, int>("nova", 3);
];

WeightedList<string, int> wl = new(data);
```

> [!Note]
> If creating a new `WeightedList` with new data, this is more verbose than simply using tuples. However, this is useful (and necessary) for handling existing `WeightedItem`s or collections containing them.

Finally, data can also be passed in as a dictionary. In this case, each key-value pair represents an item, where its key is the item’s value, and its value is the item’s weight (mildly confusing with words, but perfectly intuitive with code!).

```cs
Dictionary<string, int> data = new {
    ["sup"] = 2,
    ["nova"] = 3
};

WeightedList<string, int> wl = new(data);
```

The table below summarises the different ways to pass in data.

<table>
  <tr>
    <th> format </th>
    <td> <code>params</code> </td>
    <td> <code>IEnumerable</code> </td>
    <td> <code>Dictionary</code> </td>
  </tr>
  <tr>
    <td> <code>WeightedItem</code> </td>
    <td> <pre lang="csharp">WeightedList<string, int> wl = new(
    new WeightedItem<string, int>("sup", 2),
    new WeightedItem<string, int>("nova", 3)
) </pre>
    </td>
  </tr>
  <tr>
    <td> tuple </td>
    <td> <pre lang="csharp"><code>WeightedList<string, int> wl = new(
    (2, "sup"),
    (3, "nova")
) </code></pre>
    </td>
  </tr>
  <tr>
    <td> <code>KeyValuePair</code> </td>
    <td> – </td>
  </tr>
</table>		


<br>


## Indexing

Perhaps the most confusing aspect of a `WeightedList` is how its indexing works. Compared with (the current implementation in) Python, this is further complicated by the support for non-integer indices. For now, let’s tackle how indexing a `WeightedList` with integer weights works.

> [!Tip]
> If you haven’t read the [rationale](../rationale.md), now would be a good time to do so, since it illustrates the underlying concept behind the following implementation.
