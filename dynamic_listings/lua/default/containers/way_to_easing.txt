-- ...
-- We calculate the tweening factor
local factor = linearTween(time, 0, 1, duration)
-- Now we change the property we're tweening
object.property = property_original_value + (destination_value - property_original_value) * factor
