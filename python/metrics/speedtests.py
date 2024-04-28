from timeit import timeit

from .._aliases_ import *


class SpeedTest:
  '''Tests for speed performance metrics.'''

  def __init__(self):
    self._attrs = dir(self)

  def _load_(self):
    '''Find the tests to run.'''

    return (
      (each, getattr(self, each)) for each in self._attrs
      if not each.startswith("_")
    )

  def test_append(self):
    test = {"l": [], "wl": WL()}

    return {
      "list": timeit("l.append('sup')", globals = test),
      "wl": timeit("wl.append('sup')", globals = test),
    }
