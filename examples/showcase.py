from weightedlist import WeightedList

loot = WeightedList(
  (100, "common"),
  (40, "uncommon"),
  (20, "rare"),
  (10, "epic"),
  (4, "legendary"),
  (1, "mythical")
)
# this is why the weight comes first - then the weights and items line up nicely!

print(f"""Drop rates: {"\n".join(
  f"{item.rarity} {item.value}" for item in loot
)}""")

# select between 7 to 15 items
pile = loot.select(7, 15)
print(f"Loot obtained: {'\n'.join(pile)}")

rate = WeightedList(
  (20, "awesome!"),
  (20, "pretty good!"),
  (20, "not bad!"),
  (10, "fantastic!"),
  (4, "really lucky!"),
  (2, "insane!")
)

print(rate.select())