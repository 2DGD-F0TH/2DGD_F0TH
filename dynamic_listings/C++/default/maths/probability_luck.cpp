#include <algorithm>

enum Rarity {EPIC, RARE, UNCOMMON, COMMON};
// Our probabilities, from least to most common
int pool[4][2] = {
    {EPIC, 5},
    {RARE, 15},
    {UNCOMMON, 30},
    {COMMON, 50},
};

// Our "luck stat": each point gives 1% more chance to get a higher-tier item
int LUCK = 25;

// We cap the Luck stat at 100, the limit is 100% epic items
LUCK = std::min(LUCK, 100);

// We "overload" the prize pool, making the sum go over 100%
int overloaded_pool[4][2];
int overload_factor = 0;

for (int i = 0; i < 4; i++){
    int new_prob = pool[i][1] + LUCK;
    // We accumulate the overload factor for further calculation
    overload_factor = overload_factor + new_prob;
    overloaded_pool[i][0] = pool[i][0];
    overloaded_pool[i][1] = new_prob;
}

// We calculate how much we "overloaded" the prize pool
overload_factor = overload_factor - 100;

// We rebalance the prizes to a total of 100, from most to least common
int rebalanced_pool[4][2];
// We need to start from the most common, which means we will iterate backwards
for (int i = 3; i >= 0; i--){
    const int item = overloaded_pool[i][0];
    const int probability = overloaded_pool[i][1];
    // This will be modified later, if the pool is "overloaded"
    int new_probability = probability;
    // If the prize pool is still "overloaded"
    if (overload_factor > 0){
        // We calculate a "discharge factor" of sorts
        int value_to_remove = std::min(probability, overload_factor);
        // We reduce our "overload"
        overload_factor = overload_factor - value_to_remove;
        // And put the new probability for the class
        new_probability = probability - value_to_remove;
    }
    // We append the new pool item
    rebalanced_pool[i][0] = item;
    rebalanced_pool[i][1] = new_probability;
}
