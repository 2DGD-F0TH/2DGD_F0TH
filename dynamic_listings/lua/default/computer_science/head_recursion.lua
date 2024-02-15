local function count_forwards(n)
    -- Stop condition
    if (n == 0) then
        -- If we don't do this, we won't print 0
        print(n)
        return
    end
    -- Recursive call
    count_forwards(n-1)
    -- Procedure
    print(n)
end
