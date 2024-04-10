// Our probabilities, from least to most common
let pool = [
  ["EPIC", 5],
  ["RARE", 15],
  ["UNCOMMON", 30],
  ["COMMON", 50],
];

// Our "luck stat": each point gives 1% more chance to get a higher-tier item
let luck = 25;

// We cap the Luck stat at 100, the limit is 100% epic items
luck = Math.min(luck, 100);

// We "overload" the prize pool, making the sum go over 100%
let overloadedPool = pool.map(tier => [tier[0], tier[1] + luck]);
// We calculate how much we "overloaded" the prize pool
let overloadFactor = luck * pool.length;

// We rebalance the prizes to a total of 100, from most to least common
let rebalancedPool = [];
// We need to start from the most common, which means we will iterate backwards
for (let i = overloadedPool.length - 1; i >= 0; i--){
  const item = overloadedPool[i][0];
  const probability = overloadedPool[i][1];
  // This will be modified later, if the pool is "overloaded"
  let newProbability = probability;
  // If the prize pool is still "overloaded"
  if (overloadFactor > 0){
      // We calculate a "discharge factor" of sorts
      let valueToRemove = Math.min(probability, overloadFactor);
      // We reduce our "overload"
      overloadFactor -= valueToRemove;
      // And put the new probability for the class
      newProbability = probability - valueToRemove;
  }
  // We append the new pool item
  rebalancedPool.push([item, newProbability]);
}