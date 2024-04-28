from dataclasses import dataclass
from timeit import timeit

from .._aliases_ import *


class TestMetrics:
  meta: dict = {"start": 0, "stop": 0, "delta": 0}
  tests: dict[str, dict[str, float]] = {}


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
    test = {"l": [], "wl": WL(), "wi": WI("sup", 1)}

    return {
      "_metric": "count",
      **{
        10**i: {
          "list": timeit("l.append(wi)", globals = test, number = 10**i),
          "wl": timeit("wl.append(wi)", globals = test, number = 10**i),
        }
        for i in range(3, 8)
      }
    }
