# Rationale
> This document acts as an explanation behind the purpose of this module, and a few implementation choices whose reasons may not seem apparent at first.

Too many projects have called for weighted randomization (where each item within a collection has a different chance of being chosen).

A quick way of replicating this would be having duplicate items:

```py
rl = ["sup", "sup", "nova", "nova", "nova", "nova", "nova"]
random.choice(rl)
```

However, this is extremely inefficient and impractical, especially for very disproportionate weights.

`random.choices()` is also rather clunky, as the weights have to be passed in separately. For example, in order to store the items and weights together:

```py
rl = [("sup", 2), ("nova", 7), ("shard", 13)]
random.choices([i[0] for i in rl], [i[1] for i in rl])
```

Of course, this can all be solved with a quick function, which will suffice in some scenarios.

The aim of `WeightedList` though, is to conveniently abstract all this functionality into a single, integrated class, with a plethora of different methods to manipulate items.

Random selection uses the `self.select()` method, which incorporates a multitude of random selections all in one.

```py
wl = WeightedList((2, "sup"), (7, "nova"), (13, "shard"))
wl.select()
wl.select(2)
wl.select(1, 10)
wl.select(2, 20, 2)
wl.select(7, replace = False)
wl.select(depth = True).weight
```