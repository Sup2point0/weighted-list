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
    assert WL(*seq([val])) == e

  assert e == WL(**{"sup": 2, "nova": 1})


def test_getitem():
  t = _default_()

  for i in range(0, 2):
    assert t[i] == "sup"
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


def test_iter():
  t = _default_()
  e = [WI("sup", 2), WI("nova", 3), WI("shard", 5)]
  assert [item for item in t] == e
  assert list(t) == e

  assert WL(*e) == _default_()


def test_properties():
  t = _default_()

  assert t.values == ["sup", "nova", "shard"]
  assert len(t.values) == 3

  assert t.weights == [2, 3, 5]
  assert sum(t.weights) == 10


def test_add():
  t = _default_()
  e = WL(*_default_(), *_default_())
  assert t + WL() == t
  assert t + t == e


def test_multiply():
  t = _default_()
  e = WL(*_default_(), *_default_())
  assert t * 1 == t
  assert t * 2 == e
  assert t * 4 == e * 2


def test_append():
  t = WL()
  assert t.append("sup") == WL("sup")
  assert t.append("sup") == WL("sup", "sup")
  assert t.append((3, "nova")) == WL("sup", "sup", nova = 3)
  assert t.append(WI("shard", 7)) == WL("sup", "sup", nova = 3, shard = 7)


def test_extend():
  t = WL()
  e = _default_()
  assert t.extend(e) == e


def test_merge():
  t = WL("sup")
  tt = WL("nova")
  e = WL("sup", "nova")
  assert t.merge(tt) == e
  assert t | tt == e
  assert tt.merge(t) != e
  assert tt | t != e

  t = WL("sup")
  e = WL(sup = 2)
  assert t.merge(t) == e
  assert t | t == e
  assert t.merge(t).merge(t) != e
  assert t | t | t != e
  assert t.merge(e) == e.merge(t)
  assert t | e == e | t


def test_norm():
  t = WL(sup = 2, nova = 3)
  e = WL(sup = 2/5, nova = 3/5)
  assert t.normalise() == e

  e = WP(sup = 4/5, nova = 6/5)
  assert t.normalise(2) == e
