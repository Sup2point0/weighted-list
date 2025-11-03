import itertools as it
import math

from python import WeightedList as WL, WeightedItem as WI


ITERS = 10 ** 2


def _default_():
  return WL(sup = 2, nova = 3, shard = 5)


def test_item():
  t = WI("sup", 1)
  assert t.value == "sup"
  assert t.weight == 1

  t = WI("sup")
  e = WI("sup", 1)
  assert t == e

  t = WI("sup", 1)
  x = WI("sup", 2)
  assert t != x
  assert t != 1
  assert t != None


def test_init():
  t = WL()

  e = WL(WI("sup", 1))

  seqs = [tuple, list, set]
  vals = ["sup", (1, "sup"), WI("sup")]
  t = it.product(seqs, vals)

  for seq, val in t:
    assert WL(*seq([val])) == e

  assert e == WL(**{"sup": 1})


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


def test_getitem():
  t = _default_()

  for i in range(0, 2):
    assert t[i].value == "sup"
  for i in range(2, 5):
    assert t[i].value == "nova"
  for i in range(5, 10):
    assert t[i].value == "shard"

  for i in range(-10, -8):
    assert t[i].value == "sup"
  for i in range(-8, -5):
    assert t[i].value == "nova"
  for i in range(-5, 0):
    assert t[i].value == "shard"


def test_setitem():
  t = _default_()

  t[0] = WI("new", 7)
  e = WL((7, "new"), (3, "nova"), (5, "shard"))
  assert t == e


def test_delitem():
  t = _default_()

  del t[0]
  e = WL((3, "nova"), (5, "shard"))
  assert t == e


def test_contains():
  t = _default_()

  assert WI("sup", 2) in t
  assert WI("nova", 3) in t
  assert WI("shard", 5) in t


def test_iter():
  t = _default_()
  e = [WI("sup", 2), WI("nova", 3), WI("shard", 5)]
  assert [item for item in t] == e
  assert list(t) == e

  assert WL(*e) == _default_()


def test_total():
  t = _default_()
  assert t.total == 10

  t = WL()
  assert t.total == 0


def test_properties():
  t = _default_()

  assert t.values == ["sup", "nova", "shard"]
  assert list(t.ivalues()) == ["sup", "nova", "shard"]

  assert t.weights == [2, 3, 5]
  assert list(t.iweights()) == [2, 3, 5]


# def test_add():
#   t = _default_()
#   e = WL(*_default_(), *_default_())
#   assert t + WL() == t
#   assert t + t == e

#   t = _default_()
#   t += t
#   e = WL(*_default_(), *_default_())
#   assert t == e
#   t += e
#   assert t == e + e


def test_multiply():
  t = _default_()
  e = WL(*_default_(), *_default_())
  assert t * 0 == WL()
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
  assert e == t.extend(e)


def test_insert():
  t = WL()
  e = WL("sup")
  assert e == t.insert(0, WI("sup"))
  
  t = WL()
  t.insert(0, WI("shard", 5))
  t.insert(0, WI("sup", 2))
  t.insert(2, WI("nova", 3))
  e = _default_()
  assert t == e


def test_select():
  t = _default_()
  e = t.values
  for i in range(ITERS):
    assert t.select() in e


def test_selects():
  t = _default_()
  e = list(t.as_raw())
  assert sorted(e) == sorted(t.selects(10, replace = False))
  e = t.values
  assert sorted(e) == sorted(t.selects(3, replace = False, unique = True))


def test_merge():
  t = WL("sup")
  tt = WL("nova")
  e = WL("sup", "nova")
  assert t.merged(tt) == e
  assert t | tt == e
  assert tt.merged(t) != e
  assert tt | t != e

  t = WL("sup")
  e = WL(sup = 2)
  assert t.merged(t) == e
  assert t | t == e
  assert t.merged(t).merged(t) != e
  assert t | t | t != e
  assert t.merged(e) == e.merged(t)
  assert t | e == e | t


def test_norm():
  t = WL(sup = 2, nova = 3)
  e = WL(sup = 2/5, nova = 3/5)
  t.normalise()
  assert t.values == e.values
  assert all(math.isclose(t.weights[i], e.weights[i]) for i in range(2))

  e = WL(sup = 4/5, nova = 6/5)
  t.normalise(2)
  assert t.values == e.values
  assert all(math.isclose(t.weights[i], e.weights[i]) for i in range(2))


def test_drop():
  t = WL(sup = 2)
  e = WL(sup = 1)

  t.drop(0)
  assert t == e
  t.drop(0)
  assert t == WL()
