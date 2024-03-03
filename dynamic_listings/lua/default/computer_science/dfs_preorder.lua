local function dfs_traverse_preorder(n)
    -- Step 1: Visit the node, in this case we print its value
    print(n.content)
    -- Step 2: We traverse the left subtree, using recursion
    if (n.left ~= nil) then
        dfs_traverse_preorder(n.left)
    end
    -- Step 3: We traverse the right subtre, using recursion
    if (n.right ~= nil) then
        dfs_traverse_preorder(n.right)
    end
end

local function main()
    local root = build_example_tree()
    dfs_traverse_preorder(root)
end
