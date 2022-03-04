-- We need a "contains" function for some checks
local function contains(list, x)
	for _, v in pairs(list) do
		if v == x then return true end
	end
	return false
end

-- We bootstrap the variables
local openSet = {}
local closedSet = {}
local currentNode = start_node
table.insert(closedSet, currentNode)  -- This will avoid re-analizing this node
repeat
    local adjacent_nodes = currentNode:getAdjacentList()
    for i = 1, #adjacent_nodes do
        local n = adjacent_nodes[i]
        if (contains(closedSet, n)) then
            -- We already analyzed this node, continue to next n
            -- Since Lua doesn't have a "continue" statement, we'll use GoTo
            goto continue
        else
            n.parent = currentNode
            if (not contains(openSet, n)) then
                n.h = getHeuristicCost(n, end_node) -- Computes the value of n's h(x)
                table.insert(openSet, n)
            end
        end
        ::continue::
    end
    if (#openSet == 0) then
        -- We exhausted all the possibilities
        break
    end
    -- Sort the openset by h(x)
    table.sort(openSet, function(a,b) return a.h < b.h end)
    -- Select a new "currentNode", which is the first by construction
    currentNode = openSet[1]
    table.remove(openSet, 1)
    table.insert(closedSet, currentNode)
until (currentNode == end_node)

if (currentNode == end_node) then
    -- We reached the end and solved the path, we need to do a stack
    -- reversal to find the path
    local finalPath = {}
    local n = end_node
    while (n ~= nil) do
        table.insert(finalPath, n)
        n = n.parent  -- We use "parent" to run the found path backwards
    end
else
    -- We cannot find a path between "start" and "end"
    -- ... Behave accordingly
end
