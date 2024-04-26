'''
Tests performance metrics.
'''

from .speedtests import SpeedTests
from .record import record


# Unfortunately since this project is a submodule within suptools, and intended to require no dependencies, it can’t use all the utility functions from there


def test_all():
  results = {}
  tests = SpeedTests.tests

  for each in tests:
    print(f">> {each}()")
    results[each] = tests[each]()

  return results


if __name__ == "__main__":
  print(">> running!")

  results = test_all()
  record(results)

  print(">> done!")
