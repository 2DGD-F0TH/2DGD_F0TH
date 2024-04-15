-- Our probabilities, from least to most common
local pool = {
    {"EPIC", 5},
    {"RARE", 15},
    {"UNCOMMON", 30},
    {"COMMON", 50},
}

-- Our "luck stat": each point gives 1% more chance to get a higher-tier item
LUCK = 25

-- We cap the Luck stat at 100, the limit is 100% epic items
LUCK = math.min(LUCK, 100)

-- We "overload" the prize pool, making the sum go over 100%
Overloaded_pool = {}
Overload_factor = 0

for i = 1, #pool do
    local new_prob = pool[i][2] + LUCK
    -- We accumulate the overload factor for further calculation
    Overload_factor = Overload_factor + new_prob;
    Overloaded_pool[i] = {pool[i][1], new_prob}
end

-- We calculate how much we "overloaded" the prize pool
Overload_factor = Overload_factor - 100

-- We rebalance the prizes to a total of 100, from most to least common
Rebalanced_pool = {}
-- We need to start from the most common, which means we will iterate backwards
for i = #pool, 0, -1 do
    local item = Overloaded_pool[i][1]
    local probability = Overloaded_pool[i][2];
    -- This will be modified later, if the pool is "overloaded"
    local new_probability = probability
    -- If the prize pool is still "overloaded"
    if Overload_factor > 0 then
        -- We calculate a "discharge factor" of sorts
        local value_to_remove = math.min(probability, Overload_factor)
        -- We reduce our "overload"
        Overload_factor = Overload_factor - value_to_remove
        -- And put the new probability for the class
        new_probability = probability - value_to_remove
    end
    -- We append the new pool item
    Rebalanced_pool[i] = {item, new_probability}
end
