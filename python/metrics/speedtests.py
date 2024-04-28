from timeit import timeit

from .._aliases_ import *


class SpeedTest:
  '''Tests for speed performance metrics.'''

  def __init__(self):
    self._attrs = vars(self)

  def _load_(self):
    '''Find the tests to run.'''

    return (
      (each, self._attrs[each]) for each in self._attrs
      if not each.startswith("_")
    )

  @ staticmethod
  def test_append():
    out = {}

    l = []
    out["list"] = timeit(lambda: l.append("sup"), "from __main__ import l")
    wl = WL()
    out["wl"] = timeit(lambda: wl.append("sup"), "from __main__ import wl")
    # fwl = FWL()
    # out["fwl"] = timeit(lambda: fwl.append("sup"), "from __main__ import fwl")

    return out
