# Rationale

This article explains the intent and purposes behind this project.

> [!NOTE]
> This article uses Python for code demonstrations.

Too many projects (of mine) have called for *weighted randomisation* – where each item in a collection has a particular (different) probability of being chosen, its *weight*. The key difference to *uniform randomisation*[^stats] is that each item does not necessarily have to have the same probability of being chosen.

[^stats]: Really bringing out the statistics terminology here, aren’t we...

For instance, a loot system in a game would have items of different rarities. A ‘biased’ die would have different probabilities of landing on each face.

A quick, and perhaps naïve, approach to emulating this behaviour would be to duplicate each item according to its desired weight.[^duplicate] Naturally, those that appear more would then be more likely to chosen.

[^duplicate]: As it happens, almost every single person I’ve shown this project to has, without fail, immediately suggested this approach.

```py
>>> items = ["sup", "sup", "nova", "nova", "nova"]
>>> random.choice(items)
'nova'
# 'nova' more likely to be chosen than 'sup'
```

The issues with this should be fairly apparent to any programmer. It’s disastrously space-inefficient, uncomfortably unreadable, and painfully unscalable. Imagine if we had this:

| rarity | probability |
| :----- | :---------- |
| Tier 2 | 84% |
| Tier 1 | 15% |
| Tier 0 | 1% |

For this, we’d have to duplicate the Tier 2 item 84 times and the Tier 1 item 15 times, just so the Tier 3 item can have a rarity of 1 in 100. And what if in an update we want to change that rarity to 1 in 200? Or a horrible fraction like 1 in 69? 

Maybe you could automate the process, eh? We could harness Python’s quick and easy list concatenation:

```py
items = 2*["sup"] + 3*["nova"]
```

Hey, not bad. But this still doesn’t work well for decimal weights, and you’re still creating a huge list when you have many items and/or really large weights.

Above all, the critical issue here is size – the more disproportionate or... floating-pointy? your weights are, the more duplicates you’ll need. That increases memory usage, and means iterating through the list will take a pretty damn long time. It would be far quicker to store the values simply alongside their weights, right? Oh hey, we can store them as tuples then. That’s not a bad idea – then we can just create a helper function to sort out randomly selecting them based on their weights.

Now Python actually already offers a way of implementing weighted randomisation, with `random.choices()`.

```py
values = ["sup", "nova"]
weights = [2, 3]

random.choices(values, weights)
```

Here, the weights are passed in separately, which is somewhat clunky. You can, however, still store the values and weights together with a bit of engineering:

```py
items = [("sup", 2), ("nova", 3)]

random.choices(
  [each[0] for each in items],
  [each[1] for each in items]
)

# or even more pythonically
random.choices(*zip(*items))
```

And that actually works pretty well, since it’s only marginally more effort to do some conversions.

With all this being said, the aim of `WeightedList` is to nicely abstract all this trouble into a single, integrated container class, which handles all of the functionality for you and can be slotted into another project with ease. What’s more, it provides a slew of other utility properties and methods, in particular many kinds of randomised selection – that being what the class was made for.

Of course, for some simple situations a list of tuples or a dictionary alongside a dedicated helper function might just be enough. But hey, even if this is totally over-engineered, over-engineering stuff is fun. And it taught me loads about Python and C#, so hooray for that!


<br>
