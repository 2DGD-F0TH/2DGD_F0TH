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
repeat
    local adjacent_nodes = currentNode:getAdjacentList()
    for i = 1, #adjacent_nodes do
        local n = adjacent_nodes[i]
        if (contains(closedSet, n)) then
            goto continue
        elseif (contains(openSet, n)) then  -- Check if this path is better
            local new_g = getPathCost(n, start_node)
            if (new_g < n.g) then
                -- We found a better path from start to currentNode
                n.parent = currentNode
                n.g = new_g
            end
        else
            n.parent = currentNode
            n.g = getPathCost(n, start_node)
            table.insert(openSet, n)
        end
        ::continue::
    end

    if (#openSet == 0) then
        -- We exhausted all possibilities
        break
    end
    -- Sort the openSet by g(x)
    table.sort(openSet, function(a, b) return a.g < b.g end)
    -- Select a new "currentNode", which is the first by construction
    currentNode = openSet[1]
    table.remove(openSet, 1)
    table.insert(closedSet, currentNode)
until(currentNode == end_node)
