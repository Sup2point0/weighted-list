'''
Tests performance metrics.
'''

from datetime import datetime

from python.metrics.speedtests import TestMetrics, SpeedTest
from python.metrics.record import record


# Unfortunately since this project is a submodule within suptools, and intended to require no dependencies, it can’t use all the utility functions from there


def test_all():
  results = TestMetrics()
  results.meta["start"] = datetime.now()

  for test, func in SpeedTest()._load_():
    results.tests[test] = func()

  results.meta["stop"] = datetime.now()
  results.meta["delta"] = datetime.fromtimestamp((results.meta["stop"] - results.meta["start"]).seconds)
  results.meta["delta"] = results.meta["stop"] - results.meta["start"]

  return results


if __name__ == "__main__":
  print(">> running!")

  results = test_all()
  record(results)

  print(">> done!")
