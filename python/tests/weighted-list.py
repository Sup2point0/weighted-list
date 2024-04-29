import itertools as it

import sys
sys.path[0] = "/".join(sys.path[0].split("/")[:-1])

from python import WeightedList as WL, WeightedItem as WI


# t = test, e = expected

def _default_():
  return WL(sup = 2, nova = 3, shard = 5)


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


def test_getitem():
  t = _default_()

  for i in range(0, 2):
    assert t[0] == "sup"
  for i in range(2, 5):
    assert t[i] == "nova"
  for i in range(5, 10):
    assert t[i] == "shard"

  for i in range(-10, -5):
    assert t[i] == "sup"
  for i in range(-5, -2):
    assert t[i] == "nova"
  for i in range(-2, 0):
    assert t[i] == "shard"


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


def test_append():
  t = WL()
  assert t.append("sup") == WL("sup")
  assert t.append("sup") == WL("sup", "sup")
  assert t.append((3, "nova")) == WL("sup", "sup", nova = 3)
  assert t.append(WI("shard", 7)) == WL("sup", "sup", nova = 3, shard = 7)
