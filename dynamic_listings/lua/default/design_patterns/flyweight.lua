-- Here we take advantage of Lua's tables
Common = {
    -- Contains the common data for a 3D Object to be replicated
    mesh = {},
    texture = nil
}

FlyWeight = {
    -- Contains only the necessary data to create an instance of the item
    common_pointer = Common,
    position = {x=0, y=0},
    scale_factor = 1
}
