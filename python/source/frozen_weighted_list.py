from __future__ import annotations

import itertools
import random

from dataclasses import dataclass
from numbers import Number
from typing import Any, Iterable, Generator, Callable
from typing import Self, NoReturn


__all__ = ["FrozenWeightedItem", "FrozenWeightedList"]

Value = Any


@ dataclass(frozen = True)
class FrozenWeightedItem:
  '''An immutable item within a `FrozenWeightedList` with a `value` and `weight`.'''

  value: Value
  weight: Number
  index: Number

  def __post_init__(self):
    if not isinstance(self.weight, Number):
      if isinstance(self.value, Number):
        raise TypeError(f"Item weights must be numerical but {type(self.weight)} was provided. Perhaps you passed the value and weight in the wrong way round?")
      else:
        raise TypeError(f"Item weights must be numerical but {type(self.weight)} was provided")

    if self.weight < 0:
      raise ValueError("item weight cannot be negative")
    if self.weight == float("inf"):
      raise ValueError("item weight cannot be infinite")

  def __repr__(self):
    return f"FrozenWeightedItem(value = {self.value}, weight = {self.weight})"

  def __iter__(self):
    return iter((self.value, self.weight))
  
  def __eq__(self, item):
    return (
      self.value == item.value and 
      self.weight == item.weight
    )


class FrozenWeightedList:
  '''An immutable list of weighted items.'''

  LikeWeightedList = Iterable[FrozenWeightedItem | tuple[Number, Value]]

  def __init__(self, *items, **ktems):
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
  def _sanitise_(self, item):
    raise NotImplementedError()

  ## CORE ##
  def __str__(self):
    return f"FrozenWeightedList(" + ", ".join(f"{item.value}: {item.weight}" for item in self) + ")"

  ## SPECIALIST METHODS ##
  def select(self, item = False) -> Value | FrozenWeightedItem:
    '''Randomly select a value, considering weights.'''
