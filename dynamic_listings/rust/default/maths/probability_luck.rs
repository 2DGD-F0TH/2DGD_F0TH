#[derive(Clone, Copy)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

// Our probabilities, from least to most common
const POOL: [(Rarity, u8); 4] = [
    (Rarity::Epic, 5),
    (Rarity::Rare, 15),
    (Rarity::Uncommon, 30),
    (Rarity::Common, 50),
];

# fn main() {
// Our "luck stat": each point gives 1% more chance to get a higher-tier item
let mut luck = 25;

// We cap the Luck stat at 100, the limit is 100% epic items
luck = u8::min(luck, 100);

// We "overload" the prize pool, making the sum go over 100%
let mut overloaded_pool: [(Rarity, u8); 4] = POOL;
let mut overload_factor = 0;

for x in 0..4 {
    let new_prob = POOL[x].1 + luck;
    // We accumulate the overload factor for further calculation
    overload_factor += new_prob;
    overloaded_pool[x].0 = POOL[x].0;
    overloaded_pool[x].1 = new_prob;
}

// We calculate how much we "overloaded" the prize pool
overload_factor = overload_factor - 100;

// We rebalance the prizes to a total of 100, from most to least common
let mut rebalanced_pool: [(Rarity, u8); 4] = POOL;
// We need to start from the most common, which means we will iterate backwards
for x in (0..=3).rev() {
    let item = overloaded_pool[x].0;
    let probability = overloaded_pool[x].1;
    // This will be modified later, if the pool is "overloaded"
    let mut new_probability = probability;
    // If the prize pool is still "overloaded"
    if overload_factor > 0 {
        // We calculate a "discharge factor" of sorts
        let value_to_remove = u8::min(probability, overload_factor);
        // We reduce our "overload"
        overload_factor = overload_factor - value_to_remove;
        // And put the new probability for the class
        new_probability = probability - value_to_remove;
    }
    // We append the new pool item
    rebalanced_pool[x].0 = item;
    rebalanced_pool[x].1 = new_probability;
}
# }
