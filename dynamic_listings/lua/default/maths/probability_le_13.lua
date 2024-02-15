local function main()
    local happened = 0
    -- Monte Carlo Method we do 10000 "extractions"
    for i = 1, 10000, 1 do
        -- Get a random number between 1 and 100
        local n = math.random(1, 100)
        if (n <= 13) then
            -- If it's less or equal than 13, we got a match
            happened = happened + 1
        end
    end
    -- We print the result
    print(happened / 10000)
end
