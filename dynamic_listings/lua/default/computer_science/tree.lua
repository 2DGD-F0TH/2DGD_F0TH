--[[
-- This is an example of a simple node structure for a tree.
-- It can be used as root or any other node
--]]
Node = {}

function Node:new(o)
    o = o or {
        value = 0,
        left = nil,
        right = nil
    }
    setmetatable(o, self)
    self.__index = self
    return o
end


local function build_example_tree()
    -- Let's build the example tree starting with the nodes
    local A = Node:new({value = "A"})
    local B = Node:new({value = "B"})
    local C = Node:new({value = "C"})
    local D = Node:new({value = "D"})
    local E = Node:new({value = "E"})
    local F = Node:new({value = "F"})
    local G = Node:new({value = "G"})
    local H = Node:new({value = "H"})
    local I = Node:new({value = "I"})
    -- Now we connect the various components (the edges)
    B.left = A
    B.right = C
    F.left = E
    D.left = B
    D.right = F
    H.right = I
    G.left = D
    G.right = H
    -- The tree is ready to be used, let's return the root (G)
    return G
end
