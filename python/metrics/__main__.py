'''
Tests performance metrics.
'''

from python.metrics.speedtests import SpeedTest
from python.metrics.record import record


# Unfortunately since this project is a submodule within suptools, and intended to require no dependencies, it can’t use all the utility functions from there


def test_all():
  results = {}
  tests = SpeedTest()

  for test, func in tests._load_():
    print(f">> {tests}()")
    results[test] = func()

  return results


if __name__ == "__main__":
  print(">> running!")

  results = test_all()
  record(results)

  print(">> done!")
