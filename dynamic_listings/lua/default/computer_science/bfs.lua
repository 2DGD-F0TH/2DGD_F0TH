local function traverse_bfs(root)
    -- We will use a queue for this algorithm, simulated with a table
    local q = {}
    -- First thing, we enqueue the root
    table.insert(q, root)
    -- Now comes the iterative part. This will keep going until
    -- the tree is completely explored.
    while (#q ~= 0) do
        -- We take the first node in the queue
        local n = table.remove(q, 1)
        -- We enqueue its children, if they exist
        if (n.left ~= nil) then
            table.insert(q, n.left)
        end
        if (n.right ~= nil) then
            table.insert(q, n.right)
        end
        -- Now we visit the current node
        print(n.content)
        -- The loop will continue with the next node in the layer,
        -- automatically start the next layer, or stop because there
        -- are no more nodes to visit.
    end
end
