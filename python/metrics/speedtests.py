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
    test = {"l": [], "wl": WL()}

    return {
      "_metric": "count",
      10 ** 6: {
        "list": timeit("l.append('sup')", globals = test),
        "wl": timeit("wl.append('sup')", globals = test),
      }
    }
