'''
Convenient creation and manipulation of weighted lists.
'''


from __future__ import annotations

from copy import deepcopy
from random import randint, choice, sample
from typing import Any, Iterable, NamedTuple


class WeightedList(list):
  '''A list of weighted items.

  Items are stored as `WeightedItem` objects, which have a `value` and `weight`. The weight of each value can be thought of as how many duplicates are stored:

  ```py
  >>> wl = WeightedList(sup = 2, nova = 5)
  >>> wl[0].value
  'sup'
  >>> wl[1].value
  'sup'
  >>> wl[2].value
  'nova'
  >>> wl[8].value
  'nova'
  ```

  This allows weighted selection and randomization to be implemented without storing excessive duplicate items (especially for very disproportionate weights), increasing readability and usability.

  ```py
  wl = WeightedList(sup = 2, nova = 5)
  wl.select()

  # equivalent to:
  rl = ["sup", "sup", "nova", "nova", "nova", "nova", "nova"]
  random.choice(rl)
  ```
  '''

  class WeightedItem:
    '''A minor inner class representing a weighted item, with a `value` and `weight` attribute.'''

    def __init__(self, value, weight = 1, /):
      self.value = value
      self.weight = round(weight)

    def __repr__(self, /):
      return "WeightedItem(" + f"{repr(self.value)}, {self.weight}" + ")"

    @ property
    def raw(self, /):
      '''A `NamedTuple` with the raw item info.'''
      return WeightedList.Item(self.value, self.weight)


  class Item(NamedTuple):
    '''A `NamedTuple` representing a weighted item.'''

    value: Any
    weight: int = 1


  def _convert_(self, item, /):
    '''A minor inner method to convert any suitable value to a `WeightedItem`.'''

    lt = lambda cls: isinstance(item, cls)
    if lt(WeightedList.WeightedItem):
      return item
    if lt(WeightedList.Item):
      return WeightedList.WeightedItem(item.value, item.weight)
    if lt(dict):
      item = item.items()
    if lt(str) or not lt(Iterable):
      return WeightedList.WeightedItem(item)

    # type checks
    try:
      weight = round(item[0])
    except:

      # order checks
      try:
        weight = round(item[1])
      except:
        raise TypeError("item weights must be numerical values")
      raise TypeError("item weights must be numerical values. Perhaps you passed the value and weight in the wrong way round?")

    # value checks
    if weight < 1: raise ValueError("item weights cannot be less than 1")
    if weight < 0: raise ValueError("item weights cannot be negative")
    
    return WeightedList.WeightedItem(item[1], item[0])


  def _index_(self, index, /, *, depth = False):
    '''A minor inner method to find the item or unweighted index at a weighted `index`.'''

    # currently does not support index slicing
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


  def _clear_(self, /):
    '''A minor inner method to remove items whose `weight` is less than 1.'''

    self.__init__(*filter(lambda i: i.weight >= 1, self))
    return self



  def __init__(self, /, *items, **ktems):
    '''Create a weighted list.

    Arguments can be passed in through many different ways.

    ### Positional Arguments
    The standard way is passing them as weight-value pairs. Note that the weight must come first (this is so that they consistently align together):

    ```py
    wl = WeightedList(
      (2, "sup"),
      (7, "nova")
    )
    ```

    Existing sequences can be converted into a `WeightedList` by unpacking their contents, as long as each item follows this format:

    ```py
    pl = [(2, "sup"), (7, "nova")]
    wl = WeightedList(*pl)
    ```

    ### Keyword Arguments
    For each kwarg, the key is the item, and the value is its weight. Keyword arguments are certainly the most readable, but note that they only allow `str` items with no spaces or duplicates:

    ```py
    wl = WeightedList(sup = 2, nova = 7)
    ```

    Exisintg mappings can be passed in by unpacking their contents too:

    ```py
    pl = {"sup": 2, "nova": 7}
    wl = WeightedList(**pl)
    ```

    All can be used in tandem if desired, although discouraged for evident reasons.
    '''

    # merge different input methods into one list
    super().__init__(
      [self._convert_(i) for i in items]
    + [self._convert_(i[::-1]) for i in ktems.items()]
    )


  def __repr__(self, /):
    return "WeightedList(" + ", ".join(f"({repr(i.value)}, {i.weight})" for i in self) + ")"


  def __str__(self, /):
    return "[" + ", ".join(f"{i.value} = {i.weight}" for i in self) + "]"


  def __getitem__(self, index, /) -> WeightedItem:
    '''Return item at `index` (considering weights).'''

    return self._index_(index, depth = True)


  def __setitem__(self, index, value, /):
    '''Set item at `index` (considering weights) to `value`.'''

    super().__setitem__(self._index_(index), self._convert_(value))


  def __delitem__(self, index, /):
    '''Delete (entire) item at `index` (considering weights).'''

    super().__delitem__(self._index_(index))


  def __contains__(self, value, /):
    '''Check if contents contains `value`.'''

    return value in (v for i in self for v in (i.value, i.weight))


  def __len__(self, /):
    '''Return total length (considering weights).'''

    return sum(i.weight for i in self)


  def __add__(self, value, /):
    '''Return list extended by each item in `value`.'''

    return deepcopy(self).extend(value)

  def __radd__(self, value, /):
    return self.__add__(value)

  def __iadd__(self, value, /):
    '''Extend list by each item in `value`.'''

    return self.extend(value)


  def __mul__(self, value, /):
    '''Return list repeated `value` times.'''

    self = deepcopy(self)
    super().__imul__(value)
    return self

  def __rmul__(self, value, /):
    return self.__mul__(value)

  def __imul__(self, value, /):
    '''Repeat list `value` times.'''

    super().__imul__(value)
    return self


  def __truediv__(self, value, /):
    '''Return list with all weights `/` `value`.'''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight / value)
    return self._clear_()


  def __floordiv__(self, value, /):
    '''Return list with all weights `//` `value`.'''

    for i in (self := deepcopy(self)):
      i.weight = int(i.weight // value)
    return self._clear_()


  def __mod__(self, value, /):
    '''Return list with all weights `%` `value`.'''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight % value)
    return self._clear_()


  def __pow__(self, value, /):
    '''Return list with all weights `**` `value`.'''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight ** value)
    return self._clear_()


  def __eq__(self, value, /):
    '''Check if all items and weights in list are equal and in equal order to those in another.'''

    if not isinstance(value, WeightedList) or len(self) != len(value):
      return False
    return [(i.value, i.weight) for i in self] == [(i.value, i.weight) for i in value]

  def __ne__(self, value, /):
    return not self == value


  def __or__(self, value, /):
    '''Return list merged with `value`.'''

    return self.merge(value)

  def __ior__(self, value, /):
    '''Merge contents with `value`.'''

    self = self.merge(value)
    return self



  def append(self, weight: int, value = None, /) -> WeightedList:
    '''Append an item with `value` and `weight`.
    
    If `value` is not provided, `weight` will become the item’s value, while its weight will default to 1. If `weight` is already a `WeightedItem` (for example, extracted from another `WeightedList`), it will just be directly appended.

    Returns the modified list for fluent chaining.
    '''

    if value:
      super().append(WeightedList.WeightedItem(value, weight))
    else:
      super().append(self._convert_(weight))
    return self


  def extend(self, /, *items: Iterable) -> WeightedList:
    '''Extend list by each iterable in `items`.
   
    Each iterable can be any type that is suitable to be converted to a `WeightedList`, the same as those that can be unpacked during instantiation.

    Returns the modified list for fluent chaining.
    '''

    if isinstance(items, dict):
      super().extend(self._convert_(i[::-1]) for i in items.items())
    else:
      super().extend(self._convert_(i) for i in deepcopy(items))
    return self


  def insert(self, index, value, /) -> WeightedList:
    '''Insert `value` before `index` (considering weights).

    The value will be inserted before the *entire* item at `index`.

    `value` should be a suitable value that can be converted to a weighted item, the same as those provided during instantiation (note that the weight always comes first). If it only consists of a single value, that value’s weight will default to 1.

    Returns the modified list for fluent chaining.
    '''

    super().insert(self._index_(index), self._convert_(value))
    return self


  def remove(self, value, count = 1, /) -> WeightedList:
    '''Remove occurrence `count` of an item with `value`.

    Returns the modified list for fluent chaining.
    '''

    i = 0
    for item in self:
      if item.value == value:
        i += 1
        if i >= count:
          super().pop(i)
          break
    
    return self


  def pop(self, index = -1, /, *, drop = False) -> WeightedItem:
    '''Pop and return item at `index` (considering weights).
    
    If drop is `False`, the entire item will be removed.

    If drop is `True`, the weight of the item will be decreased by 1 instead. If it then reaches 0, the item will be removed.
    '''

    if drop and self[index].weight > 1:
      self[index].weight -= 1
      return self[index - 1]
    else:
      return super().pop(self._index_(index))


  def clear(self, /) -> WeightedList:
    '''Clear list contents.
    
    Returns the list for fluent chaining.
    '''

    super().clear()
    return self
  
  
  def sort(self, /) -> WeightedList:
    '''Return a copy with items sorted by weight in ascending order.'''

    return sorted(self, key = lambda i: i.weight)


  def select(self, /, *count, replace = True, depth = False) -> WeightedItem | list[WeightedItem]:
    '''Return a random item(s), considering weights.

    This is a convenience method that incorporates many different forms of randomized selection.

    ### Single Selection
    When called with no arguments, a single item is selected at random and returned:

    ```py
    >>> wl = WeightedList(sup = 2, nova = 7)
    >>> wl.select()
    'nova'
    >>> wl.select()
    'sup'
    >>> wl.select()
    'nova'
    # 'nova' has a higher weight, so has a higher chance of being selected
    ```

    ### Multiple Selection
    Multiple items can be selected at once with `count`. In this case, a list of values is returned, as opposed to just a single value:

    ```py
    >>> wl.select(2)
    ['nova', 'nova']
    ```

    If 2 arguments are passed for `count`, a random integer is chosen between those 2 numbers (inclusive). This then becomes the number of items selected:

    ```py
    # selects anywhere between 1 to 10 items
    >>> wl.select(1, 10)
    # equivalent to:
    >>> wl.select(random.randint(1, 10))
    ```

    If 3 arguments are passed, the third becomes the step of the range:

    ```py
    # selects 2, 4, ...18 or 20 items
    >>> wl.select(2, 20, 2)
    # equivalent to:
    >>> wl.select(random.choice(range(2, 22, 2)))
    ```

    Note that if `1` is passed, a list will still be returned:

    ```py
    >>> wl.select()
    'nova'
    >>> wl.select(1)
    ['nova']
    ```

    ### Selection Depth
    By default, selections return the value stored within the selected item(s). If `depth` is `True`, then the `WeightedItem` itself will be returned.

    ```py
    >>> wl.select()
    'nova'
    >>> wl.select(depth = True)
    WeightedItem('nova', 7)
    ```

    ### Selection Replacement
    By default, selections are made with replacement (i.e. items can be selected more than once). If `replace` is `False`, then items will be removed from the pool upon selection (note that this does not affect the original list in any way).

    ```py
    >>> wl.select(20, replace = False)
    ['nova', 'nova', 'sup', 'nova', 'nova', 'nova', 'nova', 'sup', 'nova']
    # only 9 items have been selected
    ```
    '''

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


  def merge(self, /, *items: Iterable) -> WeightedList:
    '''Merge contents with each iterable in `items`, increasing an item’s weight if it already exists, else appending it.

    Sequences can be `WeightedList`s, or any compatible iterable (those that can be unpacked during instantiation).
    
    ```py
    >>> wl = WeightedList(sup = 2, nova = 7)
    >>> vl = WeightedList(sup = 20, shard = 13)
    >>> wl.merge(vl)
    WeightedList(('sup', 22), ('nova', 7), ('shard', 13))
    # the 2 'sup' items and their weights have been combined
    ```

    Multiple lists can be merged simultaneously if needed.

    ```py
    wl.merge(wl, vl, vl)
    ```

    Returns the modified list for fluent chaining.
    '''

    for item in items:
      swap = isinstance(item, dict)
      for i in item.items() if swap else item:
        i = self._convert_(i[::-1] if swap else i)
        if i.value in self:
          self.index(i.value, depth = True).weight += i.weight
        else:
          self.append(i)

    return self


  def index(self, value, /, count = 1, *, depth = False) -> int:
    '''Return index (considering weights) of occurrence `count` of `value`.'''

    i, idx = 0, 0
    for item in self:
      if item.value == value:
        idx += 1
        if idx >= count:
          return item if depth else i
      i += item.weight


  def identify(self, weight: int, /, count = 1, *, depth = False) -> int:
    '''Return index (considering weights) of occurrence `count` of an item with `weight`.'''

    i, idx = 0, 0
    for item in self:
      if item.weight == weight:
        idx += 1
        if idx >= count:
          return item if depth else i
      i += item.weight


  def count(self, value, /) -> int:
    '''Return number of occurrences of `value`.'''

    return sum(i.value == value for i in self)


  def frequency(self, weight: int, /) -> int:
    '''Return number of occurrences of items with `weight`.'''

    return sum(i.weight == weight for i in self)


  def total(self, value, /) -> int:
    '''Return total weight of all occurrences of `value`.'''

    return sum(i.weight * (i.value == value) for i in self)


  def shift(self, dist: int = 1, /) -> WeightedList:
    '''Return a copy with all value-weight pairings shifted by `dist` (right). Values remain in place, while weights move.'''

    self = deepcopy(self)
    dist = dist % len(self)
    self.__init__(*zip(self.weights[-dist:] + self.weights[:-dist], self.values))
    return self


  def shuffle(self, /) -> WeightedList:
    '''Return a copy with all value-weight pairings shuffled. Values remain in place, while weights move.'''

    self = deepcopy(self)
    self.__init__(*zip(sample(self.weights, len(self.weights)), self.values))
    return self


  def increment(self, value = 1, /) -> WeightedList:
    '''Return a copy with all weights increased by `value`.
    
    Note that `value` can be negative.
    '''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight + value)
    return self._clear_()


  def augment(self, value: int | float, /) -> WeightedList:
    '''Return a copy with all weights multiplied by `value`.'''

    for i in (self := deepcopy(self)):
      i.weight = round(i.weight * value)
    return self._clear_()


  def deviate(self, factor: int | float, /) -> WeightedList:
    '''Raise all weights to the power of `factor`. Non-integer weights will be rounded.

    Returns the modified list for fluent chaining.
    '''

    # reciprocal checking (otherwise all weights would be rounded to 0)
    if factor < 0:
      raise ValueError("factor cannot be negative")

    for i in self:
      i.weight = round(i.weight ** factor)
    return self._clear_()


  def replace(self, value, new, /) -> WeightedList:
    '''Return a copy with all occurrences of `value` replaced with `new`.'''

    for i in (self := deepcopy(self)):
      if i.value == value:
        i.value = new
    return self


  def variate(self, weight: int = None, new: int = None, /) -> WeightedList:
    '''Return a copy with all occurrences of `weight` replaced with `new`.
    
    If `new` is not passed, each occurrence will be replaced with a random weight between the list’s minimum and maximum weights.

    If `weight` is not passed, all weights will be replaced.
    '''

    # test convert to check if weight is valid
    self._convert_((new, None))
    
    for i in (self := deepcopy(self)):
      if i.weight == weight or weight == None:
        i.weight = new or randint(self.min().weight, self.max().weight)
    
    return self


  def extrapolate(self, /) -> list[Any]:
    '''Extract contents as a raw `list` of each `value` repeated `weight` times.
    
    ```py
    >>> wl = WeightedList(sup = 2, nova = 5)
    >>> wl.extrapolate()
    ['sup', 'sup', 'nova', 'nova', 'nova', 'nova', 'nova']
    ```

    This can also be accessed through the `raw` property.
    '''

    return [v for i in self for v in [i.value] * i.weight]


  def cluster(self, /, size: int = None) -> WeightedList:
    '''Return a copy with contents clustered such that each item’s weight does not exceed `size`. If size is not passed, all duplicate items will be merged.

    Items whose weights exceed `size` are split into multiple items:

    ```py
    >>> wl = WeightedList(sip = 1, sup = 7, nova = 13)
    >>> wl.cluster(5)
    WeightedList(('sup', 1), ('sup', 5), ('sup', 2), ('nova', 5), ('nova', 5), ('nova', 3))
    # the total weight of each item remains as it was before
    ```
    '''

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



  def min(self, /) -> WeightedItem | list[WeightedItem]:
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


  def max(self, /) -> WeightedItem | list[WeightedItem]:
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


  def median(self, /) -> Any:
    '''Return median item, considering weights. If there are 2, the former is returned.'''

    return self[len(self) // 2 - 1]


  def average(self, /) -> float:
    '''Return average (arithemetic mean) of item weights.'''

    return float(len(self) / super().__len__())



  @ property
  def values(self, /) -> list[Any]:
    '''Extract a `list` of stored values.'''

    return [i.value for i in self]


  @ property
  def weights(self, /) -> list[int]:
    '''Extract a `list` of stored weights.'''

    return [i.weight for i in self]


  @ property
  def items(self, /) -> list[NamedTuple]:
    '''Extract contents as a raw `list` of value-weight `NamedTuple`s.'''

    return [WeightedList.Item(i.value, i.weight) for i in self]


  @ property
  def list(self, /) -> list[list[Any, int]]:
    '''Extract a `list` of stored value-weight pairs.'''

    return [[i.value, i.weight] for i in self]


  @ property
  def dict(self, /) -> dict[Any, int]:
    '''Extract a `dict` of stored value-weight pairs.'''

    data = {}
    for i in self:
      i = i.value
      if i in data:
        data[i] += i.weight
      else:
        data[i] = i.weight
    
    return data


  @ property
  def raw(self, /) -> list[Any]:
    '''Extract contents as a raw `list` of each `value` repeated `weight` times.'''

    return [v for i in self for v in [i.value] * i.weight]
