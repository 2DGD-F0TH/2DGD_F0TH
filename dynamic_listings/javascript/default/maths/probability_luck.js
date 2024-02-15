// Our probabilities, from least to most common
let pool = [
    ["EPIC", 5],
    ["RARE", 15],
    ["UNCOMMON", 30],
    ["COMMON", 50],
];

// Our "luck stat": each point gives 1% more chance to get a higher-tier item
let LUCK = 25;

// We cap the Luck stat at 100, the limit is 100% epic items
LUCK = Math.min(LUCK, 100);

// We "overload" the prize pool, making the sum go over 100%
let overloaded_pool = [];
let overload_factor = 0;

for (let i = 0; i < pool.length; i++){
    let new_prob = pool[i][1] + LUCK;
    // We accumulate the overload factor for further calculation
    overload_factor = overload_factor + new_prob;
    overloaded_pool.push([pool[i][0], new_prob]);
}

// We calculate how much we "overloaded" the prize pool
overload_factor = overload_factor - 100;

// We rebalance the prizes to a total of 100, from most to least common
let rebalanced_pool = [];
// We need to start from the most common, which means we will iterate backwards
for (let i = pool.length - 1; i > 0; i--){
    const item = pool[i][0];
    const probability = pool[i][1];
    // This will be modified later, if the pool is "overloaded"
    let new_probability = probability;
    // If the prize pool is still "overloaded"
    if (overload_factor > 0){
        // We calculate a "discharge factor" of sorts
        let value_to_remove = Math.min(probability, overload_factor);
        // We reduce our "overload"
        overload_factor = overload_factor - value_to_remove;
        // And put the new probability for the class
        new_probability = probability - value_to_remove;
    }
    // We append the new pool item
    rebalanced_pool.push([item, new_probability]);
}
