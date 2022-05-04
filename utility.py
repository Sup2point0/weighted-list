'''
Various assorted utility functions involving `WeightedList`s.
'''


from weightedlist import WeightedList


def display(wl: WeightedList, /, *, rarity = True) -> str:
  '''Print the contents of a `WeightedList` in an inventory-like fashion.'''
  return "\n".join(f"x{i.weight} {repr(i.value)}{f'  ({round(100 * i.weight / len(wl), 2)}%)' * rarity}" for i in wl)

def rarity(wl: WeightedList, /, *, dp = 4) -> str:
  '''Print the rarities (selection probabilities) of each item in a `WeightedList`.'''
  return {i: round(i.weight / len(wl), dp) for i in wl}