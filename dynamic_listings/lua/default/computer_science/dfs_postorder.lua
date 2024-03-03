local function dfs_traverse_postorder(n)
    -- Step 1: We traverse the left subtree, using recursion
    if (n.left ~= nil) then
        dfs_traverse_postorder(n.left)
    end
    -- Step 2: We traverse the right subtre, using recursion
    if (n.right ~= nil) then
        dfs_traverse_postorder(n.right)
    end
    -- Step 3: Visit the node, in this case we print its value
    print(n.content)
end

local function main()
    local root = build_example_tree()
    dfs_traverse_postorder(root)
end
