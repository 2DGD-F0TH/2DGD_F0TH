local function count_backwards(n)
    -- Stop condition
    if (n == 0) then
        -- If we don't do this, we won't print 0
        print(n)
        return
    end
    -- Procedure
    print(n)
    -- Recursive call
    count_backwards(n-1)
end
