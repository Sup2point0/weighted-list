'''
Convenient creation and manipulation of weighted lists.
'''


from copy import deepcopy
from random import randint, choice, sample
from typing import Any, Iterable, NamedTuple


class WeightedList(list):
  '''A list of weighted items.'''

  class WeightedItem:
    def __init__(self, value, weight = 1):
      self.value = value
      self.weight = round(weight)

    def __repr__(self):
      return "WeightedItem(" + f"{repr(self.value)}, {self.weight}" + ")"

    @ property
    def raw(self):
      return WeightedList.Item(self.value, self.weight)


  class Item(NamedTuple):
    value: Any
    weight = 1


  def _convert_(self, item):
    lt = lambda cls: isinstance(item, cls)
    if lt(WeightedList.WeightedItem):
      return item
    if lt(WeightedList.Item):
      return WeightedList.WeightedItem(item.value, item.weight)
    if lt(dict):
      item = item.items()
    if lt(str) or not lt(Iterable):
      return WeightedList.WeightedItem(item)

    try:
      weight = round(item[0])
    except:
      try:
        weight = round(item[1])
      except:
        raise TypeError("item weights must be numerical values")
      raise TypeError("item weights must be numerical values. Perhaps you passed the value and weight in the wrong way round?")
    if weight < 1: raise ValueError("item weights cannot be less than 1")
    if weight < 0: raise ValueError("item weights cannot be negative")
    
    return WeightedList.WeightedItem(item[1], item[0])


  def _index_(self, index, *, depth = False):
    if isinstance(index, slice):
      raise NotImplementedError("support for index slicing will be added in a future version")
    
    i, idx = 0, 0

    if index < 0:
      for item in reversed(self):
        if not item.weight > 0:
          continue
        i -= item.weight
        idx -= 1
        if index >= i:
          return item if depth else idx
    
    else:
      for item in self:
        if not item.weight > 0:
          continue
        i += item.weight
        if i > index:
          return item if depth else idx
        idx += 1
    
    raise IndexError("index out of range")


  def _clear_(self):
    self.__init__(*filter(lambda i: i.weight >= 1, self))
    return self



  def __init__(self, *items, **ktems):
    '''Create a weighted list.'''

    super().__init__([self._convert_(i) for i in items] + [self._convert_(i[::-1]) for i in ktems.items()])


  def __repr__(self):
    return "WeightedList(" + ", ".join(f"({repr(i.value)}, {i.weight})" for i in self) + ")"


  def __str__(self):
    return "[" + ", ".join(f"{i.value} = {i.weight}" for i in self) + "]"


  def __getitem__(self, index):
    return self._index_(index, depth = True)


  def __setitem__(self, index, value):
    super().__setitem__(self._index_(index), self._convert_(value))


  def __delitem__(self, index):
    super().__delitem__(self._index_(index))


  def __contains__(self, value):
    return value in (v for i in self for v in (i.value, i.weight))


  def __len__(self):
    return sum(i.weight for i in self)


  def __add__(self, value):
    return deepcopy(self).extend(value)

  def __radd__(self, value):
    return self.__add__(value)

  def __iadd__(self, value):
    return self.extend(value)


  def __mul__(self, value):
    self = deepcopy(self)
    super().__imul__(value)
    return self

  def __rmul__(self, value):
    return self.__mul__(value)

  def __imul__(self, value):
    super().__imul__(value)
    return self


  def __truediv__(self, value):
    for i in (self := deepcopy(self)):
      i.weight = round(i.weight / value)
    return self._clear_()


  def __floordiv__(self, value):
    for i in (self := deepcopy(self)):
      i.weight = int(i.weight // value)
    return self._clear_()


  def __mod__(self, value):
    for i in (self := deepcopy(self)):
      i.weight = round(i.weight % value)
    return self._clear_()


  def __pow__(self, value):
    for i in (self := deepcopy(self)):
      i.weight = round(i.weight ** value)
    return self._clear_()


  def __eq__(self, value):
    if not isinstance(value, WeightedList) or len(self) != len(value):
      return False
    return [(i.value, i.weight) for i in self] == [(i.value, i.weight) for i in value]

  def __ne__(self, value):
    return not self == value


  def __or__(self, value):
    return self.merge(value)

  def __ior__(self, value):
    self = self.merge(value)
    return self



  def append(self, weight, value = None):
    '''Append an item with `value` and `weight`.'''

    if value:
      super().append(WeightedList.WeightedItem(value, weight))
    else:
      super().append(self._convert_(weight))
    return self


  def extend(self, *items):
    '''Extend list by each iterable in `items`.'''

    if isinstance(items, dict):
      super().extend(self._convert_(i[::-1]) for i in items.items())
    else:
      super().extend(self._convert_(i) for i in deepcopy(items))
    return self


  def insert(self, index, value):
    '''Insert `value` before `index` (considering weights).'''

    super().insert(self._index_(index), self._convert_(value))
    return self


  def remove(self, value, count = 1):
    '''Remove occurrence `count` of an item with `value`.'''

    i = 0
    for item in self:
      if item.value == value:
        i += 1
        if i >= count:
          super().pop(i)
          break
    
    return self


  def pop(self, index = -1, *, drop = False):
    '''Pop and return item at `index`.'''

    if drop and self[index].weight > 1:
      self[index].weight -= 1
      return self[index - 1]
    else:
      return super().pop(self._index_(index))


  def clear(self):
    '''Clear list contents.'''

    super().clear()
    return self
  
  
  def sort(self):
    '''Return a copy with items sorted by weight in ascending order.'''

    return sorted(self, key = lambda i: i.weight)


  def select(self, *count, replace = True, depth = False):
    '''Return a random item(s), considering weights.'''

    if count:
      if len(count) > 1:
        if not all(isinstance(i, int) for i in count[:3]):
          raise TypeError("count should be an integer")
        if len(count) > 2:
          count = choice(range(count[0], count[1] + count[2], count[2]))
        else:
          count = randint(count[0], count[1])
      else:
        count = count[0]
        if not isinstance(count, int):
          raise TypeError("count should be an integer")
      if count < 1:
        raise ValueError("count cannot be less than 1")
      
      self = deepcopy(self)
      items = []
      
      for i in range(count):
        if len(self) < 1:
          break

        i = randint(0, len(self) - 1)
        item = self[i]
        items.append(item if depth else item.value)

        if not replace:
          item.weight -= 1
          if item.weight < 1:
            item.weight += 1
            del self[i]
      
      return items
    
    item = choice(self)
    return item if depth else item.value


  def merge(self, *items):
    '''Merge contents with each iterable in `items`, increasing an item’s weight if it already exists, else appending it.'''

    for item in items:
      swap = isinstance(item, dict)
      for i in item.items() if swap else item:
        i = self._convert_(i[::-1] if swap else i)
        if i.value in self:
          self.index(i.value, depth = True).weight += i.weight
        else:
          self.append(i)

    return self


  def index(self, value, count = 1, *, depth = False):
    '''Return index (considering weights) of occurrence `count` of `value`.'''

    i, idx = 0, 0
    for item in self:
      if item.value == value:
        idx += 1
        if idx >= count:
          return item if depth else i
      i += item.weight


  def identify(self, weight, count = 1, *, depth = False):
    '''Return index (considering weights) of occurrence `count` of an item with `weight`.'''

    i, idx = 0, 0
    for item in self:
      if item.weight == weight:
        idx += 1
        if idx >= count:
          return item if depth else i
      i += item.weight


  def count(self, value):
    '''Return number of occurrences of `value`.'''

    return sum(i.value == value for i in self)


  def frequency(self, weight):
    '''Return number of occurrences of items with `weight`.'''

    return sum(i.weight == weight for i in self)


  def total(self, value):
    '''Return total weight of all occurrences of `value`.'''

    return sum(i.weight * (i.value == value) for i in self)


  def shift(self, dist = 1):
    '''Return a copy with all value-weight pairings shifted by `dist` (right). Values remain in place, while weights move.'''

    self = deepcopy(self)
    dist = dist % len(self)
    self.__init__(*zip(self.weights[-dist:] + self.weights[:-dist], self.values))
    return self


  def shuffle(self):
    '''Return a copy with all value-weight pairings shuffled. Values remain in place, while weights move.'''

    self = deepcopy(self)
    self.__init__(*zip(sample(self.weights, len(self.weights)), self.values))
    return self


  def increment(self, value = 1):
    '''Return a copy with all weights increased by `value`.
    
    Note that `value` can be negative.
    '''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight + value)
    return self._clear_()


  def augment(self, value):
    '''Return a copy with all weights multiplied by `value`.'''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight * value)
    return self._clear_()


  def deviate(self, factor):
    '''Raise all weights to the power of `factor`. Non-integer weights will be rounded.'''

    # reciprocal checking (otherwise all weights would be rounded to 0)
    if factor < 0:
      raise ValueError("factor cannot be negative")

    for i in self:
      i.weight = round(i.weight ** factor)
    return self._clear_()


  def replace(self, value, new):
    '''Return a copy with all occurrences of `value` replaced with `new`.'''

    for i in (self := deepcopy(self)):
      if i.value == value:
        i.value = new
    return self


  def variate(self, weight = None, new = None):
    '''Return a copy with all occurrences of `weight` replaced with `new`.'''

    # test convert to check if weight is valid
    self._convert_((new, None))
    
    for i in (self := deepcopy(self)):
      if i.weight == weight or weight == None:
        i.weight = new or randint(self.min().weight, self.max().weight)
    
    return self


  def extrapolate(self):
    '''Extract contents as a raw `list` of each `value` repeated `weight` times.'''

    return [v for i in self for v in [i.value] * i.weight]


  def cluster(self, size = None):
    '''Return a copy with contents clustered such that each item’s weight does not exceed `size`. If size is not passed, all duplicate items will be merged.'''

    if size:
      if not isinstance(size, int):
        raise TypeError("size must be a positive integer")
      if size < 1:
        raise ValueError("size cannot be below 1")

    new = WeightedList()
    for i in self:
      if size:
        while i.weight > size:
          i.weight -= size
          new.append(size, i.value)
        new.append(i.weight, i.value)
      else:
        new.merge([i])

    return new



  def min(self):
    '''Return item(s) with lowest weight. If multiple tie, a list with each is returned.'''

    i = self[0].weight
    log = []

    for item in self:
      if item.weight < i:
        i = item.weight
        log = [item]
      elif item.weight == i:
        log.append(item)

    return log if len(log) > 1 else log[0]


  def max(self):
    '''Return item(s) with highest weight. If multiple tie, a list with each is returned.'''

    i = 0
    log = []

    for item in self:
      if item.weight > i:
        i = item.weight
        log = [item]
      elif item.weight == i:
        log.append(item)

    return log if len(log) > 1 else log[0]


  def median(self):
    '''Return median item, considering weights. If there are 2, the former is returned.'''

    return self[len(self) // 2 - 1]


  def average(self):
    '''Return average (arithemetic mean) of item weights.'''

    return float(len(self) / super.__len__())



  @property
  def values(self):
    '''Extract a `list` of stored values.'''

    return [i.value for i in self]


  @property
  def weights(self):
    '''Extract a `list` of stored weights.'''

    return [i.weight for i in self]


  @property
  def items(self):
    '''Extract contents as a raw `list` of value-weight `NamedTuple`s.'''

    return [WeightedList.Item(i.value, i.weight) for i in self]


  @property
  def list(self):
    '''Extract a `list` of stored value-weight pairs.'''

    return [[i.value, i.weight] for i in self]


  @property
  def dict(self):
    '''Extract a `dict` of stored value-weight pairs.'''

    data = {}
    for i in self:
      i = i.value
      if i in data:
        data[i] += i.weight
      else:
        data[i] = i.weight
    
    return data


  @property
  def raw(self):
    '''Extract contents as a raw `list` of each `value` repeated `weight` times.'''

    return [v for i in self for v in [i.value] * i.weight]
