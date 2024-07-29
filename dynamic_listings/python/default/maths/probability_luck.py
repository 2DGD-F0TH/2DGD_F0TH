# Our probabilities, from least to most common
pool: list[tuple[str, int]] = [
    ("EPIC", 5),
    ("RARE", 15),
    ("UNCOMMON", 30),
    ("COMMON", 50),
]

# Our "luck stat": each point gives 1% more chance to get a higher-tier item
LUCK = 25

# We cap the Luck stat at 100, the limit is 100% epic items
LUCK = min(LUCK, 100)

# We "overload" the prize pool, making the sum go over 100%
overloaded_pool: list[tuple[str, int]] = []

for item, probability in pool:
    overloaded_pool.append((item, probability+LUCK))

# We calculate how much we "overloaded" the prize pool
overload_factor: int = sum(probability for _, probability in overloaded_pool) - 100

# We rebalance the prizes to a total of 100, from most to least common
rebalanced_pool: list[tuple[str, int]] = []
# We need to start from the most common, which means we will iterate backwards
for item, probability in overloaded_pool[::-1]:
    # This will be modified later, if the pool is "overloaded"
    new_probability: int = probability
    # If the prize pool is still "overloaded"
    if overload_factor > 0:
        # We calculate a "discharge factor" of sorts
        value_to_remove: int = min(probability, overload_factor)
        # We reduce our "overload"
        overload_factor -= value_to_remove
        # And put the new probability for the class
        new_probability: int = probability - value_to_remove
    # We append the new pool item
    rebalanced_pool.append((item, new_probability))
