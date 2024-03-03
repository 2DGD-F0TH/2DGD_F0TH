local function dfs_traverse_inorder(n)
    -- Step 1: We traverse the left subtree, using recursion
    if (n.left ~= nil) then
        dfs_traverse_inorder(n.left)
    end
    -- Step 2: Visit the node, in this case we print its value
    print(n.content)
    -- Step 3: We traverse the right subtre, using recursion
    if (n.right ~= nil) then
        dfs_traverse_inorder(n.right)
    end
end

local function main()
    local root = build_example_tree()
    dfs_traverse_inorder(root)
end
