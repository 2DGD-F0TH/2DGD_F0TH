Polygon = {}

function Polygon:new(o)
    -- This is an example constructor
    o = o or {vertices = {}}
    setmetatable(o, self)
    self.__index = self
    return o
end

function Polygon:calculate_bounding_box()
    -- This function calculates the bounding box
    -- -------------------------
    -- First we create and bootstrap the variables
    local xmin = self.vertices[1].x
    local xmax = self.vertices[1].x
    -- [[
    -- ..
    -- see the bounding box algorithm for the full version
    -- ..
    -- ]]
    -- We build our bounding box
    local boundingBox = Rectangle:from_points(A, C)
    -- and return it
    return boundingBox
end

function Polygon:do_fanning()
    --[[
    -- This function iterates over the vertices and returns
    -- an array of triangles corresponding to the "fan triangulation"
    --]]
    -- We fix the "base" of the fan on the first vertex
    local root_vertex = self.vertices[1]
    local temp_triangles = {}
    -- Now we iterate through all the other vertices
    for j = 2, #self.vertices do
        -- j goes from the third vertex, to the last
        -- j - 1 goes from the second to the second to last
        table.insert(temp_triangles, Triangle:from_points(root_vertex, j - 1, j))
    end
    -- In the end, we will have the triangles array, we can just return it
    return temp_triangles
end
