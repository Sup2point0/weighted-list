# <h1 align="center"> `WeightedList` </h1>

Convenient creation and manipulation of weighted lists.

## Contents
- [Purposes](#Purposes)
  - [Features](#Features)
- [Utilization](#Utilization)
  - [Installation](#Installation)
  - [Implementation](#Implementation)
- [Requirements](#Requirements)
  - [Compatibility](#Compatibility)
  - [Dependencies](#Dependencies)
- [License](#License)
- [Upcoming](#Upcoming)
- [Contribute](#Contribute)
- [Notes](#Notes)

## Purposes
Mainly intended for weighted randomization, but could also come in useful for:
- frequency distributions
- inventories and loot generation
- months and dates

### Features
- all the functionality of a regular `list`
- compact storage of weighted items
- various convenience functions to manipulate weights
- methods without side effects for flexibility
- inbuilt conversion to other data types


## Utilization

### Installation
The module is not installable as a package; instead, just download the `weightedlist.py` file and upload it to your project, or copy and paste the code directly.

All of the functionality is within the single `WeightedItem` class, so you can simply import it, and you’re ready to go!

```py
from weightedlist import WeightedList
```

It may be uploaded to PyPI in future.

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


## Requirements

### Compatibility
Made in and for Python 3.10.

An [alternative](variants/unannotated) version with reduced type hinting can be found in [variants](variants), which resolves all of the compatibility issues.

### Dependencies
Type hinting
- `__future__.annotations`
- `typing.Any`, `typing.Iterable`

Functionality
- `typing.NamedTuple`
- `copy.deepcopy`

Randomization
- `random.randint`
- `random.choice`
- `random.sample`


## License
This project is licensed under the MIT license. You’re free to use it however you wish (although some credit would be cool).

## Contribute
Any feedback, suggestions or improvements are definitely welcome!

## Upcoming
- support for slice indexing
- item rarity
- standard deviation

## Notes
Yep, I know I break loads of Python conventions, but that’s just how I like to code. Anyway, most of them can probably changed with a simple `change all occurrences`. Apologies for any inconvenience.
