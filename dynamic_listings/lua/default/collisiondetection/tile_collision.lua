TILE_WIDTH = 32
TILE_HEIGHT = 32

Rectangle = {}

function Rectangle:new(o)
    -- This is an example constructor
    o = o or {corner=Point:new(), width=0, height=0}
    setmetatable(o, self)
    self.__index = self
    return o
end

local function convert_pixels_to_tile(x, y)
    -- Converts a point into tile coordinates
    local tile_x = math.floor(x / TILE_WIDTH)
    local tile_y = math.floor(y / TILE_HEIGHT)
    return {tile_x, tile_y}
end

-- We assume the player is falling, so no check will be shown here
local points_to_check = {
    Point:new({player.corner.x, player.corner.y + player.height}),
    Point:new({player.corner.x + player.width, player.corner.y + player.height}),
}
for i = 1, #points_to_check do
    local point = points_to_check[i]
    local detected_tile_coordinates = convert_pixels_to_tile(point.x, point.y)
    local detected_tile = get_tile(detected_tile_coordinates[0], detected_tile_coordinates[1])
    if (AABB(player, detected_tile.rectangle)) then
        -- React to the collision
        -- ...
    end
end
