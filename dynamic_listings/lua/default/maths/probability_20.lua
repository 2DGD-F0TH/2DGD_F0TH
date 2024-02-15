local function main()
    local happened = 0
    -- Monte Carlo Method we do 10000 "extractions"
    for i = 1, 10000, 1 do
        -- Get a random number between 1 and 5
        local n = math.random(1, 5)
        if (n == 1) then
            -- If it's 1, we have a match!
            happened = happened + 1
        end
    end
    -- We print the result
    print(happened / 10000)
end
