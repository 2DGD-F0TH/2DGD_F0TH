-- Operators can be treated as functions, that means you can
-- Assign them to a variable.

-- This...
--

if (a == b and c == d) then
    -- Do something...
end

-- Is equivalent to this

local complex_condition = (a == b and c == d)

if (complex_condition) then
    -- ...
end

-- ---------------8<---------------

-- Also this...

local function thing(a, b)
    if (a == b) then
        return true
    else
        return false
    end
end

-- Is equivalent to...

local function thing(a, b)
    return a == b
end
