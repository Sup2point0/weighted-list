from timeit import timeit

from ._aliases_ import *


class SpeedTests:
  '''Tests for speed performance metrics.'''

  @ property
  def tests(self):
    '''A generator of tests to be run.'''

    attrs = vars(self)

    return (
      (each, attrs[each]) for each in attrs
      if not each.startswith("__")
    )

  def test_append():
    return {
      "list": timeit(lambda: l.append("sup"), "l = []"),
      "wl": timeit(lambda: l.append((1, "sup"), "wl = WL()"),
      "fwl": None,
    }
