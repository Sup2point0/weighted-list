<h1 align="center"> <code> WeightedList </code> </h1>

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
> For the full rationale behind this project, see [rationale](rationale.md).

Mainly intended for *weighted randomisation*, where each element can have a different chance of being selected (its weight).

The prime example of this is lootbox or reward systems in games, where items have different rarities.


<br>


## Features

- Compact storage of weighted items
- Randomised selection with optional constraints
- Convenience methods to manipulate values and weights
- Separate methods with and without side effects for flexibility
- Conversions from and to a wide range of other data types

### Python
- Subclass of `list` with all its regular functionality

### C#
- Generic class with type parameters for the values and their weights
- Supports non-integer weights
- Supports LINQ querying

### Future
- Slice indexing[^slice]
- Negative weights[^negative]

[^slice]: Difficult in Python, and even more difficult in C# with non-integer weights.
[^negative]: Questionable feature, likely not gonna be added.


<br>


## Usage

The project is not available as a package.[^package] Instead, just download the relevant files, or copy and paste the code directly.

[^package]: I don't think it’s a large enough project to warrant an entire package, when you could just copy and paste the code directly.

> [!Tip]
> Walkthroughs and specimens for each language can be found in their respective folders.

### Python
All you need is the [`weightedlist.py`](Python/weightedlist.py) file, which contains the `WeightedList` class with all the functionality. Simply import it, and you’re ready to go!

```py
from weightedlist import WeightedList
```

See [walkthrough](Python/walkthrough.md) for a tutorial, or [examples](Python/examples.md) for examples.

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
-->

### C#
All the code is contained within the [`WeightedList.cs`](C-sharp/WeightedList/WeightedList.cs) file. You might also need the [`WeightedList.csproj`](C-sharp/WeightedList/WeightedList.csproj) file.

If you want the entire solution, you can download the repo and extract the [`C-sharp/`](C-sharp/) folder. This also contains tests to verify that everything is working.

For a tutorial and examples, see [resources](C-sharp/resources/).


<br>


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

- Due to the nature of weighted indexing, random access has a time complexity of $O(n)$, where $n$ is the number of elements in the list.
  - However, optimising this further without significant sacrifices in space complexity (which is already decently hefty) appears unviable.


<br>


## License
This project is licensed under the MIT license, so feel free to use it however you wish (although some credit would be cool!).


<br>


## Questions

### Why did you create this?
Back when I was picking up the ropes of Python, I was working on a project which featured randomisation, and, like any game developer, I thought it’d be cool to give each outcomes different probabilities of occurring. At first, I achieved this behaviour by duplicating items, but I quickly realised the numerous issues with this.

And so, I set out to write my own class, which I’d never really needed to do up until that point. I thought it’d be a great exercise in learning Python – and it very much was, teaching me tons about object-oriented programming, dunder methods, generators, etc. It was also my first experience of conscientiously writing code that wasn’t exclusively for myself, which helped me understand the importance of consistency and clarity, and above all, documentation.

A couple years later, I’ve come back to do the same in C#, this time also adding several features I always intended to add but never did – especially non-integer weights, which allows the class to truly embrace its usage as representing probabilities. Trying to translate Python into C# was an interesting experience,[^translate] and helped highlight some important differences between the languages that I would otherwise not have found out.

[^translate]: This was not exactly the way I created the project in C#, but the Python implementation certainly laid out a general framework and was influential in some design decisions.

### Is this even useful?
I mean yeah, a whole several-hundred-lines class to handle one thing might be a bit overkill. But it’s far more convenient to have it all packaged this way into a single portable file that can easily be slotted into other projects. Regardless, I’ve used my own code[^surprise] in at least 2 major projects ([PENGUIN](https://github.com/Sup2point0/PENGUIN) and [Algorhythm](https://Sup2point0/Algorhythm)), so I can definitely say it’s been useful to me!

[^surprise]: To my own surprise, somewhat.

### Why are the source files several hundred lines long?
1, documentation; 2, line breaks; 3, extra functionality. Particularly documentation. That stuff just *eats* the line count. Also, implementing something as complex as an enumerable container requires a lot of methods, operators and interfaces, both in Python and C#. And in C# you've even got overloading to account for as well.

### Is the code that optimised?
I have tried to ensure everything is implemented as efficiently as possible, but I cannot guarantee every single part is perfectly optimised, and I haven’t gone to the extremes of timing different approaches. Operations that may take longer than expected will likely have that mentioned in their documentation (docstrings for Python, XML comments for C#).

### Why is your Python code not compliant to PEP 8?
I have my own particular preferences when it comes to coding in Python, which I explain fully [here](https://github.com/Sup2point0/Assort/blob/origin/~writing/Python%20Syntax.md).


<br>


## Contribute

Any feedback, suggestions or improvements are definitely welcome!


<br>


<!-- what you lookin at? -->
