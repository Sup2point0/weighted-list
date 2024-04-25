from __future__ import annotations

import itertools
import random
from copy import deepcopy
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
    return iter([self.value, self.weight])
  
  def __eq__(self, item):
    return (
      self.value == item.value and 
      self.weight == item.weight
    )


class WeightedList:
  '''A list of weighted items.
  '''

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
  def values(self) -> Generator[Value, None, None]:
    '''...'''

    return (item.value for item in self)

  @ property
  def weights(self) -> Generator[Number, None, None]:
    '''...'''

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

  def _index_(self, index, *, depth = False) -> Number:
    '''Find the unweighted index corresponding to a weighted index. If `depth`, return the item instead of the index.'''

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
    
    raise IndexError("WeightedList index out of range")

  ## CORE ##
  def __str__(self):
    return f"WeightedList(" + ", ".join(f"{item.value}: {item.weight}" for item in self) + ")"
  
  def __eq__(self, other: Any):
    return (
      isinstance(other, WeightedList) and
      all(left == right for left, right in zip(self, other))
    )
  
  def __ne__(self, other: Any):
    return (
      not isinstance(other, WeightedList) or
      any(left != right for left, right in zip(self, other))
    )

  ## ITERABLE METHODS ##
  def __getitem__(self, index) -> WeightedItem:
    return self._index_(index, depth = True)
  
  def __setitem__(self, index, item: WeightedItem):
    self[index] = self._sanitise_(item)

  def __delitem__(self, index):
    super().__delitem__(self._index_(index))

  def __len__(self):
    return sum(self.weights)

  def __contains__(self, item: WeightedItem):
    return any(each == item for each in self)
  
  ## OPERATORS ##

  ## LIST METHODS ##
  def append(self, item: WeightedItem | tuple[Number, Value]) -> Self:
    '''Add an item to the end of the list.'''

    super().append(self._sanitise_(item))
    return self

  def extend():
    '''...'''

  def insert(self,
    index: Number,
    item: WeightedItem | tuple[Number, Value],
  ) -> Self:
    '''...'''

    super().insert(self._index_(index), self._sanitise_(item))
    return self
  
  def clear(self) -> Self:
    '''Clear contents of the list.'''

    super().clear()
    return self

  ## SPECIALIST METHODS ##
  def select(self) -> WeightedItem:
    '''...
    '''

  def selectval(self) -> Value:
    '''...
    '''

  def selects(self, replace = False, unique = False):
    '''...
    '''

  def clean(self) -> WeightedList:
    '''Remove all items with zero or negative weight.'''

    for item in reversed(self):
      if 0 >= item.weight:
        self.remove(item)

  def normalise(self, factor: Number = 1) -> WeightedList:
    '''Scale all item weights such that they sum to 1.'''

    t = self.total_weights()

    for item in self:
      item.weight *= factor / t

    return self

  def normalised(self) -> WeightedList:
    '''Return a copy of the list with `self.normalise()` applied.'''

    copy = deepcopy(self)
    copy.normalise()
    return copy

  ## DATA METHODS
  def as_raw(self,
    loop: Callable[[Number], int] = round,
  ) -> Generator[Any, None, None]:
    '''Return an iterator which iterates over each item in the weighted list a number of times equal to its weight.

    If item weights are not integers, they will be rounded using the inbuilt `round()`. Alternatively, a different function `loop` can be provided to specify how they should be handled.'''

    return (
      item for item in self
      for i in range(loop(item.weight))
    )

  def as_dict(self) -> dict[Any, Number]:
    '''Get a `dict` representation of the weighted list.

    Note that duplicate keys are collapsed with their weights added.'''

    out = {}

    for value, weight in self:
      try:
        out[value] += weight
      except KeyError:
        out[value] = weight

    return out
