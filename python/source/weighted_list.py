from __future__ import annotations

import itertools
from copy import deepcopy
from math import floor
from random import random, choices, shuffle

from collections.abc import Mapping
from numbers import Number
from typing import Any, Iterable, Generator, Callable
from typing import Self, NoReturn
from warnings import warn


__all__ = ["WeightedItem", "WeightedList"]

Value = Any


class WeightedItem:
  '''An item within a `WeightedList` with a `value` and `weight`.'''

  def __init__(self, value: Value, weight: Number = 1):
    if not isinstance(weight, Number):
      if isinstance(value, Number):
        raise TypeError(f"Item weights must be numerical but {type(weight)} was provided. Perhaps you passed the value and weight in the wrong way round?")
      else:
        raise TypeError(f"Item weights must be numerical but {type(weight)} was provided")

    if weight < 0:
      raise ValueError("item weight cannot be negative")
    if weight == float("inf"):
      raise ValueError("item weight cannot be infinite")

    self.value = value
    self.weight = weight

  def __repr__(self):
    return f"WeightedItem(value = {self.value}, weight = {self.weight})"

  def __iter__(self):
    return iter((self.value, self.weight))
  
  def __eq__(self, item):
    return isinstance(item, WeightedItem) and (
      self.value == item.value and 
      self.weight == item.weight
    )

  def __hash__(self):
    return hash(repr(self))


class WeightedList(list):
  '''A list of weighted items.

  All methods that modify the list return the modified instance for fluent chaining, unless they return an otherwise specified object. Hence this is allowed:

  ```py
  >>> wl = (WeightedList()
        .append(WeightedItem("sup"))
        .insert(0, WeightedItem("nova"))
        .merge()
  ```

  Some methods that modify the list have 2 variants, one that acts in-place (on the original instance) or out-of-place (on a deep copy). In this case, their name reflects their nature:
  - in-place: *present tense* (`merge` `normalise`)
  - out-of-place: *present perfect* (`merged` `normalised`)
  '''

  LikeWeightedItem = tuple[Number, Value]
  LikeWeightedList = Iterable[WeightedItem | LikeWeightedItem] | Mapping[Value, Number]

  def __init__(self, *items, **ktems):
    '''Create a weighted list.
    '''

    super().__init__(
      itertools.chain(
        (self._sanitise_(item) for item in items),
        (self._sanitise_((item[1], item[0])) for item in ktems.items()),
      )
    )

  ## PROPERTIES ##
  @ property
  def total(self) -> Number:
    '''Total weight of all items in the list.'''

    return sum(self.iweights())

  @ property
  def values(self) -> list[Value]:
    '''Return values of each item in the list.'''

    return list(self.ivalues())

  def ivalues(self) -> Generator[Value, None, None]:
    '''Return `self.values` as a generator.'''

    return (item.value for item in self)

  @ property
  def weights(self) -> list[Number]:
    '''Return weights of each item in the list.'''

    return list(self.iweights())

  def iweights(self) -> Generator[Number, None, None]:
    '''Return `self.weights` as a generator.'''

    return (item.weight for item in self)
  
  ## INTERNAL ##
  def _sanitise_(self, item) -> WeightedItem:
    '''Convert an input `item` to a suitable `WeightedItem`.'''

    if isinstance(item, WeightedItem):
      return item
    if isinstance(item, str) or not isinstance(item, Iterable):
      return WeightedItem(item)
    if isinstance(item, dict):
      return WeightedItem(*item.items())

    return WeightedItem(item[1], item[0])

  def _index_(self, index: Number, *, depth = False) -> Number | WeightedItem:
    '''Find the unweighted index corresponding to a weighted index. If `depth`, return the item instead of the index.'''

    i, idx = 0, 0

    if not self:
      return 0

    if index < 0:
      for item in reversed(self):
        if not item.weight > 0:
          print("Warning: Item with negative weight encountered in WeightedList")
          continue

        i -= item.weight
        idx -= 1
        if index >= i:
          return item if depth else idx
    
    else:
      for item in self:
        if not item.weight > 0:
          warn("Warning: Item with negative weight encountered in WeightedList")
          continue

        i += item.weight
        if i > index:
          return item if depth else idx
        idx += 1
    
    raise IndexError(f"WeightedList index out of range (tried accessing {index} but list is length {self.total})")

  ## CORE ##
  def __repr__(self):
    return f"WeightedList(" + ", ".join(f"({item.weight}, {repr(item.value)})" for item in self) + ")"
  
  def __str__(self):
    return f"WeightedList(" + ", ".join(f"{repr(item.value)}: {item.weight}" for item in self) + ")"
  
  def __eq__(self, other: Any):
    return (
      isinstance(other, WeightedList) and
      list(self) == list(other)
    )
  
  def __ne__(self, other: Any): ### TODO comparison between different lengths
    return (
      not isinstance(other, WeightedList) or
      list(self) != list(other)
    )
  
  def __bool__(self):
    return any(self.iweights())

  ## ITERABLE METHODS ##
  def __getitem__(self, index: Number | slice) -> WeightedItem:
    if isinstance(index, slice):
      raise NotImplementedError("Slice indexing is currently unsupported for WeightedLists")
    
    return self._index_(index, depth = True)
  
  def __setitem__(self, index, item: WeightedItem) -> NoReturn:
    super().__setitem__(self._index_(index), self._sanitise_(item))

  def __delitem__(self, index) -> NoReturn:
    super().__delitem__(self._index_(index))

  def __contains__(self, item: WeightedItem) -> bool:
    return any(each == item for each in self)
  
  ## OPERATORS ##
  def __add__(self, other: WeightedList) -> Self:
    return deepcopy(self).extend(other)

  def __iadd__(self, other: WeightedList) -> Self:
    return self.extend(other)

  def __mul__(self, value: int) -> Self:
    new = deepcopy(self)
    new *= value
    return new

  def __rmul__(self, value: int) -> Self:
    return self.__mul__(value)

  def __imul__(self, value: int) -> Self:
    super().__imul__(value)
    return self

  def __or__(self, other: WeightedList) -> Self:
    return self.merged(other)
  
  def __ror__(self, other: WeightedList) -> Self:
    return self.merged(other)

  def __ior__(self, other: WeightedList) -> Self:
    return self.merge(other)

  ## LIST METHODS ##
  def append(self, item: WeightedItem | tuple[Number, Value]) -> Self:
    '''Add an item to the end of the list.'''

    super().append(self._sanitise_(item))
    return self

  def extend(self, items: Iterable | dict) -> Self:
    '''...'''

    if isinstance(items, dict):
      super().extend(self._sanitise_(each[::-1]) for each in items.items())
    else:
      super().extend(self._sanitise_(each) for each in items)
    return self

  def insert(self,
    index: Number,
    item: WeightedItem | tuple[Number, Value],
  ) -> Self:
    '''Insert `item` before the (entire) item at `index` (weighted).'''

    super().insert(self._index_(index), self._sanitise_(item))
    return self

  def pop(self, index: Number = -1) -> WeightedItem:
    '''Remove and return (entire) item at (weighted) `index`.'''

    return super().pop(self._index_(index))
  
  def clear(self) -> Self:
    '''Clear contents of the list.'''

    super().clear()
    return self

  def copy(self) -> WeightedList:
    '''Return a shallow copy of the list.'''

    return super().copy()
  
  def deepcopy(self) -> WeightedList:
    '''Return a deep copy of the list.'''

    return deepcopy(self)

  ## SPECIALIST METHODS ##
  def select(self, *, entire = False, drop = False) -> Value | WeightedItem:
    '''Randomly select a random item from the list. If `entire`, return the `WeightedItem` itself.

    If `drop` is `1`, the item’s weight will be decremented by 1. If `drop` is `True`, the entire item will be removed from the list.
    '''

    idx = floor(self.total * random())
    out = (
      self.pop(idx) if drop is True else
      self.drop(idx) if drop == 1 else
      self[idx]
    )
    return out if entire else out.value

  def selects(self, count, *, replace = False, unique = False) -> list[Value]:
    '''Randomly select `count` values from the list.
    '''

    return list(self.iselects(count, replace = replace, unique = unique))

  def iselects(self, count, *, replace = False, unique = False) -> Generator[Value, None, None]:
    '''Return `self.selects()` as a generator.'''

    if unique or not replace:
      # NOTE this does not mutate the original list
      self = deepcopy(self).merge()
      drop = True if unique else 1 if not replace else 0

    for i in range(count):
      yield self.select(drop = drop)
  
  def merge(self, other: WeightedList | LikeWeightedList = None) -> Self:
    '''Merge the list with another WeightedList-like iterable, increasing an item’s weight if it already exists, otherwise appending it.
    
    If nothing is provided, the list will instead merge items in itself so that there are no duplicate values.
    '''

    if other is None:
      self = WeightedList().merge(self)
    else:
      for each in other:
        found = list(self.find(lambda item: item.value == each.value))
        if found:
          found[0].weight += each.weight
        else:
          self.append(each)

    return self

  def merged(self, other: WeightedList | LikeWeightedList = None) -> WeightedList:
    '''Return a copy of the list with `self.merge()` applied.'''

    return deepcopy(self).merge(other)

  def find(self,
    predicate: Callable[[WeightedItem], bool],
  ) -> Generator[WeightedItem, None, None]:
    '''Find all items in the list that fulfil `predicate`.'''

    return (item for item in self if predicate(item))

  def count(self, item: WeightedItem) -> int:
    '''Calculate total weight of all occurrences of an item in the list.'''

    return sum(each.weight for each in self if each == item)

  def shuffle(self) -> Self:
    '''Shuffle value-weight pairings in the list, with values remaining in place while the weights move.'''

    self.__init__(zip(
      self.values,
      shuffle(self.iweights())
    ))

  def normalise(self, factor: Number = 1) -> Self:
    '''Scale all item weights such that they sum to 1.'''

    t = self.total

    for item in self:
      item.weight *= factor / t

    return self

  def normalised(self) -> WeightedList:
    '''Return a copy of the list with `self.normalise()` applied.'''

    return deepcopy(self).normalise()

  def remove(self,
    predicate: Callable[[WeightedItem], bool],
  ) -> Self:
    '''Remove items from the list which fulfil `predicate`.'''

    for item in reversed(self):
      if predicate(item):
        del item
  
  def drop(self, index: Number = -1) -> WeightedItem:
    '''Decrement the weight of item at (weighted) `index` by 1, and return the item with the decreased weight.
    
    If the weight of item isn't greater than 1, the item will be removed and returned with a weight of 0.'''

    item = self[index]
    
    if item.weight > 1:
      item.weight -= 1
    else:
      self.pop(index)
      item.weight = 0
    
    return item

  def clean(self) -> Self:
    '''Remove all items with zero or negative weight.'''

    for item in reversed(self):
      if 0 >= item.weight:
        self.remove(item)

  ## DATA METHODS
  def as_raw(self,
    loop: Callable[[Number], int] = round,
  ) -> Generator[Any, None, None]:
    '''Return an iterator which iterates over each item in the weighted list a number of times equal to its weight.

    If item weights are not integers, they will be rounded using the inbuilt `round()`. Alternatively, a different function `loop` can be provided to specify how they should be handled.
    '''

    return (
      item.value for item in self
      for i in range(loop(item.weight))
    )

  def as_list(self) -> list[tuple[Number, Value]]:
    '''Get a `list` representation of the weighted list.'''

    return [(item.weight, item.value) for item in self]

  def as_dict(self) -> dict[Value, Number]:
    '''Get a `dict` representation of the weighted list.

    Note that duplicate keys are collapsed with their weights added.
    '''

    out = {}

    for value, weight in self:
      try:
        out[value] += weight
      except KeyError:
        out[value] = weight

    return out
