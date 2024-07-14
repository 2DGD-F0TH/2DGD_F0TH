factor = linear_tween(time, 0, 1, duration)
object.property = property_original_value + (destination_value - property_original_value) * factor
