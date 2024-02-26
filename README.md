<h1 align="center"> <code>WeightedList</code> </h1>

A class representing a list of weighted items, available in both Python and C#.


```py
# Python
greetings = WeightedList(
  (20, "sup"),
  (2, "salutations"),
)

print(greetings.select())
# sup
```

```cs
// C#
WeightedList<string, int> greetings = new(
    (2, "sup"), (20, "salutations")
);

Console.WriteLine(greetings.GetRandomValue());
// salutations
```


<br>


## Purposes

> [!Tip]
> For the full rationale behind this project, see <rationale.md>.

Mainly intended for *weighted randomisation*, where each element can have a different chance of being selected (its weight).

The prime example of this is lootbox or reward systems in games, where items have different rarities.


<br>


## Features

- Compact storage of weighted items
- Randomised selection with optional constraints
- Separate methods with and without side effects for flexibility
- Convenience methods to manipulate values and weights
- Conversions from and to a wide range of other data types

### Python
- Subclass of `list` with all its regular functionality

### C#
- Generic class with type parameters for the values and their weights
- Supports non-integer weights
- Supports LINQ querying

### Future
- Slice indexing[^slice]

[^slice]: Difficult in Python, and even more difficult in C# with non-integer weights.


## Usage

The project is not available as a package.[^package] Instead, just download the relevant files, or copy and paste the code directly.

[^package]: I don't think it’s a large enough project to warrant an entire package, when you could just copy and paste the code directly.

### Python
All you need is the [`weightedlist.py`](Python/weightedlist.py) file, which contains the `WeightedList` class with all the functionality. Simply import it, and you’re ready to go!

<!--
### Implementation
A `WeightedList` works just like how a `list` does, except rather than storing the values themselves, it stores `WeightedItem` objects. The value and weight of each item can be accessed through the `value` and `weight` attributes, respectively. These are passed in as pairs when instantiating the list:

```py
wl = WeightedList(
  (2, "sup"),
  (7, "nova"),
  (13, "shard"),
  ...
)
```

The `weight` of each item can be thought of as how many duplicates are stored (which would replicate the weighting mechanic):

```py
>>> wl = WeightedList(sup = 2, nova = 7)

>>> wl[0].value
'sup'
>>> wl[1].value
'sup'
>>> wl[2].value
'nova'
>>> wl[8].value
'nova'

>>> wl.select(7)
['nova', 'sup', 'nova', 'nova', 'sup', 'nova', 'nova']
# 'nova' has a higher change of being selected
```

Here’s a quick example of how a `WeightedList` could be used for weighted randomization:

```py
greetings = WeightedList()
responses = WeightedList()

name = input(f"{greetings.select()}! What’s your name?")
print(f"{greetings.select()} {name}. {responses.select()}!")
```

More examples and a full [walkthrough](examples/walkthrough.md) can be found in [examples](examples).
-->


## Compatibility

### Python
- Made in Python 3.10
- Uses `match`
- All imports are from the standard library, so there are no external dependencies

### C#
- Made in C# 12.0
  - Uses `System.Linq`


<br>


## Notes

- While the Python and C# implementations are extremely similar, they may differ very slightly in certain places. In the unlikely case that someone (other than me) uses both of them, just be aware of that.
- Due to the nature of weighted indexing, random access has a time complexity of $O(n)$, where $n$ is the number of elements in the list.
  - However, optimising this further without significant sacrifices in space complexity (which is already decently hefty) appears unviable.


<br>


## License
This project is licensed under the MIT license, so feel free to use it however you wish (although some credit would be cool!).


<br>


## Questions

### Why are the source files several hundred lines long?
1, documentation; 2, line breaks; 3, extra functionality. Particularly documentation. That stuff just *eats* the line count. Also, subclassing something as complex as an enumerable container requires a lot of methods to be implemented, both in Python and C#. And in C# you've even got overloading as well.

### Is the code that optimised?
I have tried to ensure everything is implemented as efficiently as possible, but I cannot guarantee every single part is perfectly optimised.

### Why is your Python code not compliant to PEP 8?
I have my own particular preferences when it comes to coding in Python, which I explain fully [here](https://github.com/Sup2point0/Assort/blob/origin/~writing/Python%20Syntax.md).


<br>


## Contribute

Any feedback, suggestions or improvements are definitely welcome!
