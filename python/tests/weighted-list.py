import itertools as it

import sys
sys.path[0] = "/".join(sys.path[0].split("/")[:-1])

from python import WeightedList as WL, WeightedItem as WI


t = "test"
e = "expected"


def test_item():
  t = WI("sup", 1)
  assert t.value == "sup"
  assert t.weight == 1

  t = WI("sup")
  e = WI("sup", 1)
  assert t == e


def test_init():
  t = WL()

  e = WL(WI("sup", 2), WI("nova", 1))

  seqs = [tuple, list, set]
  vals = ["sup", (1, "sup"), WI("sup")]
  t = it.product(seqs, vals)

  for seq, val in t:
    assert WL(*seq(list(val))) == e

  assert e == WL(**{"sup": 2, "nova": 1})


def test_eq():
  t = WL()
  e = WL()
  assert t == e
  assert t is not e

  t = WL("sup")
  e = WL("sup")
  assert t == e

  t = WL("sup", "nova")
  e = WL("sup", "nova")
  assert t == e

  t = WL()
  e = WL("sup")
  assert t != e


def test_bool():
  t = WL()
  assert not t

  t = WL(WI("sup"))
  assert t
