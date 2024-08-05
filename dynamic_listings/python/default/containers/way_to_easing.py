factor: float = linear_tween(time, 0, 1, duration)
our_object.property = property_original_value + (destination_value - property_original_value) * factor
