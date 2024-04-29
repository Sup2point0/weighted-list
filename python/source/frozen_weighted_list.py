from __future__ import annotations

import random
from numbers import Number
from typing import Any


__all__ = ["FrozenWeightedItem", "FrozenWeightedList"]

Value = Any


class FrozenWeightedItem:
  '''An immutable weighted item.'''

  def __init__(self, value: Value, weight: Number, *, index: int):
    self.value = value
    self.weight = weight
    self.index = index


class FrozenWeightedList:
  '''An immutable list of weighted items.'''

  def __init__(self, *items, **ktems):
    self.total_weights = 0

  ## INTERNAL ##
  def _sanitise_(self, item):
    raise NotImplementedError()

  ## SPECIALIST METHODS ##
  def select(self, item = False) -> Value | FrozenWeightedItem:
    '''Randomly select a value, considering weights.'''
