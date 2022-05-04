from weightedlist import WeightedList


# Game
answer = WeightedList(*(i + 1 for i in range(20))).select()

correct = WeightedList(
  (10, "correct"),
  (5, "yeah"),
)
incorrect = WeightedList(
  (10, "incorrect"),
  (5, "nope"),
)
punctuation = WeightedList(
  (10, "."),
  (5, "!"),
  (1, "!!"),
)

print("guess the secret number between 1 and 20")

while int(input("take a guess...\n")) != answer:
  print(incorrect.select() + punctuation.select())

print(correct.select() + punctuation.remove(".").select())


# Survey
data = WeightedList()

size = int(input("How many people are you surveying?\n"))
for i in range(size):
  key = input("Enter a chosen option\n")
  value = round(input("How many people chose this option?\n"))
  data.append(value, key)

data.cluster()
for item in data:
  print(f"{item.weight} people chose {repr(item.value)}")

print(f"The most popular option was {repr(data.max().value)}")
print(f"The least popular option was {repr(data.min().value)}")

data.sort()
print(f"The median option was {repr(data.median().value)}")